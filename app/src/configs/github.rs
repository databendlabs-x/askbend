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
pub struct GithubConfig {
    #[clap(long = "github_token", default_value_t)]
    pub github_token: String,
    #[clap(long = "llm_max_tokens", default_value_t = 100000)]
    pub llm_max_tokens: usize,
    #[clap(long = "databend_dsn", default_value_t)]
    pub databend_dsn: String,

    #[clap(long = "repos")]
    pub repos: Option<Vec<String>>,

    #[clap(long = "check_in_secs", default_value_t = 20)]
    pub check_in_secs: usize,
}

impl Debug for GithubConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("QAConfig")
            .field("github_token", &"******")
            .field("databend_dsn", &"******")
            .field("llm_max_tokens", &self.llm_max_tokens)
            .field("repos", &self.repos)
            .field("check_in_secs", &self.check_in_secs)
            .finish()
    }
}

impl Default for GithubConfig {
    fn default() -> Self {
        GithubConfig {
            github_token: "".to_string(),
            llm_max_tokens: 100000,
            databend_dsn: "".to_string(),
            repos: None,
            check_in_secs: 20,
        }
    }
}
