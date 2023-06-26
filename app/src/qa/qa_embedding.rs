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

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use llmchain::DatabendEmbedding;
use llmchain::DatabendVectorStore;
use llmchain::DocumentLoader;
use llmchain::DocumentPath;
use llmchain::DocumentSplitter;
use llmchain::MarkdownLoader;
use llmchain::MarkdownSplitter;
use llmchain::VectorStore;
use log::info;

use crate::Config;

pub struct QAEmbedding {
    conf: Config,
}

impl QAEmbedding {
    pub fn create(conf: &Config) -> Self {
        QAEmbedding { conf: conf.clone() }
    }

    /// Rebuild QA all embeddings.
    pub async fn rebuild(&self) -> Result<()> {
        let conf = self.conf.clone();
        let local_disk = llmchain::LocalDisk::create()?;
        let markdown_loader = MarkdownLoader::create(local_disk.clone());
        let directory_loader =
            llmchain::DirectoryLoader::create(local_disk).with_loader("**/*.md", markdown_loader);
        let documents = directory_loader
            .load(DocumentPath::Str(conf.qa.path.clone()))
            .await?;
        info!("Step-1: parser all files:{}", documents.len());

        let documents = MarkdownSplitter::create().split_documents(&documents)?;
        info!("Step-2: split all files to:{}", documents.len());

        let now = Instant::now();
        info!(
            "Step-3: begin embedding to table:{}.{}",
            conf.qa.database, conf.qa.table
        );
        let dsn = conf.qa.dsn.clone();
        let databend_embedding = Arc::new(DatabendEmbedding::create(&dsn));
        let databend_vector_store = DatabendVectorStore::create(&dsn, databend_embedding)
            .with_database(&conf.qa.database)
            .with_table(&conf.qa.table);
        databend_vector_store.init().await?;

        let _ = databend_vector_store.add_documents(&documents).await?;
        info!(
            "Step-3: finish embedding to table:{}.{}, cost {}",
            conf.qa.database,
            conf.qa.table,
            now.elapsed().as_secs()
        );
        Ok(())
    }
}
