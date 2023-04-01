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
use env_logger::Builder;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).format_target(false).init();

    let conf = Config::load()?;

    info!("Step-1: begin parser all markdown files");
    let file_opt = FileOperator::create(&conf.data.path);
    let files = file_opt.list()?;
    let markdowns = Markdown::parse_multiple(
        &files
            .iter()
            .map(|v| v.full_path.clone())
            .collect::<Vec<String>>(),
    )?;
    info!(
        "Step-1: finish parser all markdown files:{}, sections:{}, tokens:{}",
        files.len(),
        markdowns.all_sections(),
        markdowns.all_tokens()
    );

    let dal = DatabendDriver::connect(&conf)?;

    if conf.server.rebuild {
        info!("Step-2: begin insert to table");
        dal.insert(&markdowns).await?;
        info!("Step-2: finish insert to table");

        info!("Step-3: begin generate embedding, may take some minutes");
        dal.embedding().await?;
        info!("Step-3: finish generate embedding");
    } else {
        info!("Step-2(Insert) and Step-3(Embedding) skipped, rebuild=false");
    }

    info!(
        "Step-4: start api server {}:{}",
        conf.server.host, conf.server.port
    );
    let handler = APIHandler::create(&conf.server, dal.clone());
    handler.start().await?;

    Ok(())
}
