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

/// Config for Query Answering.
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

#[derive(Parser, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct QAConfig {
    // path
    #[clap(long = "path", default_value = "data/")]
    pub path: String,

    // database
    #[clap(long = "database", default_value_t)]
    pub database: String,
    #[clap(long = "table", default_value_t)]
    pub table: String,
    #[clap(long = "answer_table", default_value_t)]
    pub answer_table: String,
    #[clap(long = "dsn", default_value_t)]
    pub dsn: String,

    // query
    #[clap(long = "top", default_value_t = 2)]
    pub top: usize,

    // rebuild
    #[clap(long = "rebuild", default_value_t)]
    pub rebuild: bool,
}

impl Debug for QAConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("QAConfig")
            .field("path", &self.path)
            .field("database", &self.database)
            .field("table", &self.table)
            .field("answer_table", &self.answer_table)
            .field("dsn", &"******")
            .field("top", &self.top)
            .finish()
    }
}

impl Default for QAConfig {
    fn default() -> Self {
        QAConfig {
            path: "data/".to_string(),
            database: "".to_string(),
            table: "".to_string(),
            answer_table: "".to_string(),
            dsn: "".to_string(),
            top: 2,
            rebuild: false,
        }
    }
}
