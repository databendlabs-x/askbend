// Copyright 2023 Databend Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use databend_driver::DatabendConnection;
use log::info;
use tokio::time::Instant;
use tokio_stream::StreamExt;

use crate::base::escape_sql_string;
use crate::Config;
use crate::SnippetFiles;

#[derive(Clone)]
pub struct DatabendDriver {
    pub database: String,
    pub table: String,
    pub min_content_length: usize,
    pub max_content_length: usize,
    pub top: usize,
    pub prompt_template: String,
    pub conn: DatabendConnection,
}

impl DatabendDriver {
    pub fn connect(conf: &Config) -> Result<Self> {
        let conn = DatabendConnection::create(&conf.database.dsn)?;
        Ok(DatabendDriver {
            database: conf.database.database.clone(),
            table: conf.database.table.clone(),
            min_content_length: conf.query.min_content_length,
            max_content_length: conf.query.max_content_length,
            top: conf.query.top,
            prompt_template: conf.query.prompt.to_string(),
            conn,
        })
    }

    /// Insert all the values to databend cloud.
    pub async fn insert(&self, values: &SnippetFiles) -> Result<()> {
        let sql = format!(
            "INSERT INTO {}.{} (path, content) VALUES ",
            self.database, self.table
        );

        let mut val_vec = vec![];
        for snippet_file in &values.snippet_files {
            for snippet in &snippet_file.code_snippets {
                val_vec.push(format!(
                    "('{}', '{}')",
                    snippet_file.file_path,
                    escape_sql_string(snippet)
                ));
            }
        }
        let values = val_vec.join(",").to_string();

        let final_sql = format!("{} {}", sql, values);
        self.conn.exec(&final_sql).await
    }

    /// Build all the embedding which is empty.
    /// post each content to openai
    /// openai returns embedding vector
    /// update the table embedding
    pub async fn embedding(&self) -> Result<()> {
        let sql = format!(
            "UPDATE {}.{} SET embedding = ai_embedding_vector(left(content,{})) WHERE length(embedding)=0",
            self.database, self.table, self.max_content_length,
        );

        self.conn.exec(&sql).await
    }

    /// Get a similarly content.
    pub async fn query(&self, query: &str) -> Result<Vec<String>> {
        let mut similar_sections = vec![];
        let mut similar_distances = vec![];

        info!("query coming: {}", query);
        info!("distance query start");
        let now = Instant::now();
        // Retrieve sections with the highest similarity to the input query.
        {
            let sql = format!(
                "SELECT content, cosine_distance(ai_embedding_vector('{}'), embedding) AS distance FROM {}.{} WHERE length(embedding) > 0 AND length(content)>{} ORDER BY distance ASC LIMIT {}",
                escape_sql_string(query),
                self.database,
                self.table,
                self.min_content_length,
                self.top
            );

            type RowResult = (String, f32);
            let mut rows = self.conn.query_iter(&sql).await?;
            while let Some(row) = rows.next().await {
                let section_tuple: RowResult = row?.try_into()?;
                similar_sections.push(section_tuple.0);
                similar_distances.push(section_tuple.1);
            }
        }
        info!("distance query end cost:{:?}", now.elapsed().as_secs());
        info!("distance similar distances: {:?}", similar_distances);
        info!("distance similar sections: {:?}", similar_sections);

        let now = Instant::now();
        // Perform text completion if similar sections are found.
        if !similar_sections.is_empty() {
            info!("query completion start");
            let mut sections_text = similar_sections.to_vec().join(" ");

            let prompt = self.prompt_template.clone();

            // Keep the section is no larger.
            {
                let template_len = prompt.len();
                sections_text.truncate(8192 - template_len);
            }

            let prompt = prompt.replace("{{context}}", &sections_text);
            let prompt = prompt.replace("{{query}}", query);
            let prompt_sql = format!(
                "SELECT ai_text_completion('{}') as q",
                escape_sql_string(&prompt)
            );
            info!("prompt sql:{}", prompt_sql);

            type TextCompletionResult = (String,);
            let mut text_completions = vec![];
            let mut rows = self.conn.query_iter(&prompt_sql).await?;
            while let Some(row) = rows.next().await {
                let text_completion: TextCompletionResult = row?.try_into()?;
                info!("prompt completion:{}", text_completion.0);
                text_completions.push(text_completion.0);
            }

            info!("query completion end, cost:{:?}", now.elapsed().as_secs());
            return Ok(text_completions);
        }

        Ok(vec![])
    }
}
