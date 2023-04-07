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
use crate::SnippetFiles;
use crate::{remove_markdown_links, Config};

#[derive(Clone)]
pub struct DatabendDriver {
    pub database: String,
    pub table: String,
    pub anwser_table: String,
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
            anwser_table: conf.database.answer_table.clone(),
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
                    escape_sql_string(&snippet_file.file_path),
                    remove_markdown_links(&escape_sql_string(snippet))
                ));
            }
        }
        let values = val_vec.join(",").to_string();

        let final_sql = format!("{} {}", sql, values);
        self.conn.exec(&final_sql).await
    }

    pub async fn insert_answer(&self, query: &str, answer: &str) -> Result<()> {
        if self.anwser_table.is_empty() {
            return Ok(());
        }

        let sql = format!(
            "INSERT INTO {}.{} (question, answer) VALUES ('{}', '{}')",
            self.database,
            self.anwser_table,
            escape_sql_string(query),
            escape_sql_string(answer)
        );
        self.conn.exec(&sql).await
    }

    pub async fn get_embedding(&self, text: &str) -> Result<String> {
        let query = format!("SELECT ai_embedding_vector('{}')", escape_sql_string(text));
        type RowResult = (String,);
        let row = self.conn.query_row(&query).await?;

        if let Some(row) = row {
            let result: RowResult = row.try_into()?;
            Ok(result.0)
        } else {
            Ok("".to_string())
        }
    }

    pub async fn get_completion(&self, text: &str) -> Result<String> {
        let query = format!("SELECT ai_text_completion('{}')", escape_sql_string(text));
        type RowResult = (String,);
        let row = self.conn.query_row(&query).await?;

        if let Some(row) = row {
            let result: RowResult = row.try_into()?;
            Ok(result.0)
        } else {
            Ok("".to_string())
        }
    }

    pub async fn get_similar_sections(&self, query_embedding: &str) -> Result<Vec<String>> {
        let mut similar_sections = vec![];
        let mut similar_distances = vec![];

        let sql = format!(
            "SELECT content, cosine_distance({}, embedding) AS distance FROM {}.{} WHERE length(embedding) > 0 AND length(content)>{} ORDER BY distance ASC LIMIT {}",
            query_embedding,
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
        info!("distance similar distances: {:?}", similar_distances);

        Ok(similar_sections)
    }

    /// Build all the embedding which is empty.
    /// post each content to openai
    /// openai returns embedding vector
    /// update the table embedding
    pub async fn embedding(&self) -> Result<()> {
        let sql = format!(
            "UPDATE {}.{} SET embedding = ai_embedding_vector(left(concat(path, content),{})) WHERE length(embedding)=0",
            self.database, self.table, self.max_content_length,
        );

        self.conn.exec(&sql).await
    }

    /// Get a similarly content.
    pub async fn query(&self, query: &str) -> Result<Vec<String>> {
        // 1. Get the query embedding.
        let now = Instant::now();
        let query_embedding = self.get_embedding(query).await?;
        if query_embedding.is_empty() {
            return Ok(vec![]);
        }
        info!(
            "get embedding, query={}, cost={:?}",
            query,
            now.elapsed().as_secs()
        );

        // 2. Get the similar sections.
        let now = Instant::now();
        let similar_sections = self.get_similar_sections(&query_embedding).await?;
        info!(
            "get similar, query={}, sections={:?}, cost={:?}",
            query,
            similar_sections,
            now.elapsed().as_secs()
        );

        // 3. Get the sections completion.
        let completion = if similar_sections.is_empty() {
            let sections_text = similar_sections.to_vec().join(" ");
            let mut sections_text = remove_markdown_links(&sections_text);
            let prompt = self.prompt_template.clone();
            // Keep the section is no larger.
            {
                let template_len = prompt.len();
                sections_text.truncate(8192 - template_len);
            }

            let prompt = prompt.replace("{{context}}", &sections_text);
            let prompt = prompt.replace("{{query}}", query);

            let now = Instant::now();
            let context_completion = self.get_completion(&prompt).await?;
            info!(
                "get completion, query={},\nprompt={:?}\n,completion={:?},\ncost={:?}",
                query,
                prompt,
                similar_sections,
                now.elapsed().as_secs()
            );

            context_completion
        } else {
            "".to_string()
        };

        let now = Instant::now();
        self.insert_answer(query, &completion).await?;
        info!(
            "insert answer table, query={}, cost={:?}",
            query,
            now.elapsed().as_secs()
        );

        Ok(vec![completion])
    }
}
