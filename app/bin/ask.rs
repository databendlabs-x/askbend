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
use askbend::DatabendDriver;
use askbend::FileOperator;
use askbend::Markdown;
use askbend::Parse;
use askbend::RustCode;
use env_logger::Builder;
use env_logger::Env;
use log::info;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).format_target(false).init();

    let conf = Config::load()?;
    info!(
        "database:{}, table:{}",
        conf.database.database, conf.database.table
    );

    if conf.server.rebuild {
        let now = Instant::now();
        rebuild_embedding(&conf).await?;
        info!("Rebuild done, cost:{}", now.elapsed().as_secs());
    } else {
        start_api_server(&conf).await?;
    }

    Ok(())
}

/// Rebuild all embeddings.
async fn rebuild_embedding(conf: &Config) -> Result<()> {
    info!("Step-1: begin parser all markdown files");
    let file_opt = FileOperator::create(&conf.data.path, &conf.data.file_ext);
    let files = file_opt.list()?;

    let snippet_files = match conf.data.file_ext.as_str() {
        "md" => Markdown::parse_multiple(
            &files
                .iter()
                .map(|v| v.full_path.clone())
                .collect::<Vec<String>>(),
        ),

        "rs" => RustCode::parse_multiple(
            &files
                .iter()
                .map(|v| v.full_path.clone())
                .collect::<Vec<String>>(),
        ),
        _ => Err(anyhow::Error::msg(format!(
            "Only support file ext: md or rs, got:{}",
            conf.data.file_ext
        ))),
    }?;

    info!(
        "Step-1: finish parser all markdown files:{}, sections:{}, tokens:{}",
        files.len(),
        snippet_files.all_snippets(),
        snippet_files.all_tokens()
    );

    let dal = DatabendDriver::connect(conf)?;

    info!("Step-2: begin insert to table");
    dal.insert(&snippet_files).await?;
    info!("Step-2: finish insert to table");

    info!("Step-3: begin generate embedding, may take some minutes");
    dal.embedding().await?;
    info!("Step-3: finish generate embedding");

    Ok(())
}

/// Start the api server.
async fn start_api_server(conf: &Config) -> Result<()> {
    info!("Start api server {}:{}", conf.server.host, conf.server.port);
    let dal = DatabendDriver::connect(conf)?;
    let handler = APIHandler::create(&conf.server, dal.clone());
    handler.start().await?;
    Ok(())
}
