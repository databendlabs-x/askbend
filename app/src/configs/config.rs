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
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use serfig::collectors::from_env;
use serfig::collectors::from_file;
use serfig::collectors::from_self;
use serfig::parsers::Toml;

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct LogConfig {
    /// Log level <DEBUG|INFO|ERROR>
    #[clap(long = "log-level", default_value = "INFO")]
    pub level: String,

    /// Log file dir
    #[clap(long = "log-dir", default_value = "_logs")]
    pub dir: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            level: "INFO".to_string(),
            dir: "_logs".to_string(),
        }
    }
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct DataConfig {
    #[clap(long = "data-path", default_value = "data")]
    pub path: String,

    #[clap(long = "file-ext", default_value = "md")]
    pub file_ext: String,

    #[clap(
        long = "ignore-dirs",
        default_value = "rfc",
        use_value_delimiter = true
    )]
    pub ignore_dirs: Vec<String>,
}

impl Default for DataConfig {
    fn default() -> Self {
        DataConfig {
            path: "data".to_string(),
            file_ext: "md".to_string(),
            ignore_dirs: vec!["rfc".to_string()],
        }
    }
}

#[derive(Parser, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct DatabaseConfig {
    #[clap(long = "database", default_value_t)]
    pub database: String,
    #[clap(long = "table", default_value_t)]
    pub table: String,
    #[clap(long = "answer_table", default_value_t)]
    pub answer_table: String,
    #[clap(long = "dsn", default_value_t)]
    pub dsn: String,
}

impl Debug for DatabaseConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("database", &self.database)
            .field("table", &self.table)
            .field("answer_table", &self.answer_table)
            .field("dsn", &"******")
            .finish()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            database: "".to_string(),
            table: "".to_string(),
            answer_table: "".to_string(),
            dsn: "".to_string(),
        }
    }
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ServerConfig {
    #[clap(long = "host", default_value_t)]
    pub host: String,
    #[clap(long = "port", default_value_t)]
    pub port: usize,

    pub cors: Vec<String>,

    #[clap(long = "rebuild", default_value_t)]
    pub rebuild: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "".to_string(),
            port: 0,
            cors: Vec::new(),
            rebuild: false,
        }
    }
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct QueryConfig {
    #[clap(long = "min_content_length", default_value_t = 50)]
    pub min_content_length: usize,

    #[clap(long = "max_content_length", default_value_t = 8000)]
    pub max_content_length: usize,

    #[clap(long = "top", default_value_t = 2)]
    pub top: usize,

    #[clap(long = "min_distance", default_value_t)]
    pub min_distance: String,

    #[clap(long = "product", default_value_t)]
    pub product: String,

    #[clap(long = "prompt", default_value_t)]
    pub prompt: String,
}

impl Default for QueryConfig {
    fn default() -> Self {
        QueryConfig {
            min_content_length: 50,
            max_content_length: 8000,
            top: 2,
            min_distance: "".to_string(),
            product: "".to_string(),
            prompt: "".to_string(),
        }
    }
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    #[clap(flatten)]
    pub log: LogConfig,

    #[clap(flatten)]
    pub data: DataConfig,

    #[clap(flatten)]
    pub database: DatabaseConfig,

    #[clap(flatten)]
    pub server: ServerConfig,

    #[clap(flatten)]
    pub query: QueryConfig,

    #[clap(long, short = 'c', default_value_t)]
    pub config_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log: Default::default(),
            data: Default::default(),
            database: Default::default(),
            server: Default::default(),
            query: Default::default(),
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
