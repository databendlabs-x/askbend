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

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use llmchain::DatabendEmbedding;
use llmchain::DatabendLLM;
use llmchain::DatabendVectorStore;
use llmchain::DocumentRetrievalPrompt;
use llmchain::Prompt;
use llmchain::VectorStore;
use llmchain::LLM;
use log::info;

use crate::Config;

pub struct BendLLM {
    conf: Config,
}

impl BendLLM {
    pub fn create(conf: &Config) -> Self {
        BendLLM { conf: conf.clone() }
    }

    pub async fn query(&self, question: &str) -> Result<String> {
        let dsn = self.conf.qa.dsn.clone();
        let topk = self.conf.qa.top;

        info!("question: {}", question);

        // create embedding.
        let databend_embedding = Arc::new(DatabendEmbedding::create(&dsn));

        // create databend vector store.
        let databend_vector_store = DatabendVectorStore::create(&dsn, databend_embedding)
            .with_database(&self.conf.qa.database)
            .with_table(&self.conf.qa.table);
        let similarities = databend_vector_store
            .similarity_search(question, topk)
            .await?;

        info!("similarities: {:?}", similarities);

        let contexts = similarities
            .iter()
            .map(|x| format!("context:{}\nsource:{}", x.content, x.path))
            .collect::<Vec<_>>()
            .join("");

        let prompt_template = DocumentRetrievalPrompt::create().with_instructions(vec!["Present your answer in markdown format, including code snippets if have, format the code snippets with SQL type if necessary.",
                                                                                       "Do not include any links or external references in your response.\n",
                                                                                       "Do not change the code snippets.\n",
                                                                                       "Do not change the SQL syntax, please don't make up the function.\n",
                                                                                       "Do not change explain any code snippets.\n",
                                                                                       "Make the whole answer as short as possible to keep the code snippets.\n"
        ]);
        let mut input_variables = HashMap::new();
        input_variables.insert("question", question);
        input_variables.insert("contexts", &contexts);
        let prompt = prompt_template.format(input_variables)?;

        info!("prompt: {}", prompt);

        let databend_llm = DatabendLLM::create(&dsn);
        let result = databend_llm.generate(&prompt).await?;

        Ok(result.generation)
    }
}
