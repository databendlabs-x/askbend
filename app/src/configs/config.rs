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

use std::env;
use std::fmt::Debug;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use serfig::collectors::from_env;
use serfig::collectors::from_file;
use serfig::collectors::from_self;
use serfig::parsers::Toml;

use crate::configs::GithubConfig;
use crate::configs::LogConfig;
use crate::configs::QAConfig;

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ServerConfig {
    #[clap(long = "host", default_value_t)]
    pub host: String,
    #[clap(long = "port", default_value_t)]
    pub port: usize,

    pub cors: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "".to_string(),
            port: 0,
            cors: Vec::new(),
        }
    }
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    #[clap(flatten)]
    pub log: LogConfig,

    #[clap(flatten)]
    pub server: ServerConfig,

    #[clap(flatten)]
    pub qa: QAConfig,

    #[clap(flatten)]
    pub github: GithubConfig,

    #[clap(long, short = 'c', default_value_t)]
    pub config_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log: Default::default(),
            server: Default::default(),
            qa: Default::default(),
            github: Default::default(),
            config_file: "".to_string(),
        }
    }
}

impl Config {
    /// Load will load config from file, env and args.
    ///
    /// - Load from file as default.
    /// - Load from env, will override config from file.
    /// - Load from args as finally override
    pub fn load() -> Result<Self> {
        let arg_conf = Self::parse();
        let mut builder: serfig::Builder<Self> = serfig::Builder::default();

        // Load from config file first.
        {
            let config_file = if !arg_conf.config_file.is_empty() {
                arg_conf.config_file.clone()
            } else if let Ok(path) = env::var("CONFIG_FILE") {
                path
            } else {
                "".to_string()
            };

            if !arg_conf.config_file.is_empty() {
                builder = builder.collect(from_file(Toml, &config_file));
            }
        }

        // Then, load from env.
        builder = builder.collect(from_env());

        // Finally, load from args.
        builder = builder.collect(from_self(arg_conf));
        builder.build()
    }
}
