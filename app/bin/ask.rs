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
use askbend::APIHandler;
use askbend::Config;
use askbend::GithubComment;
use askbend::QAEmbedding;
use env_logger::Builder;
use env_logger::Env;
use log::info;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).format_target(false).init();

    let conf = Config::load()?;
    info!("config: {:?}", conf);

    if conf.qa.rebuild {
        let now = Instant::now();
        let qa_embedding = QAEmbedding::create(&conf);
        qa_embedding.rebuild().await?;
        info!("QA rebuild done, cost:{}", now.elapsed().as_secs());
    } else {
        let github_comments = GithubComment::create(&conf);
        github_comments.start();

        start_api_server(&conf).await?;
    }

    Ok(())
}

/// Start the api server.
async fn start_api_server(conf: &Config) -> Result<()> {
    info!("Start api server {}:{}", conf.server.host, conf.server.port);
    let handler = APIHandler::create(conf);
    handler.start().await?;
    Ok(())
}
