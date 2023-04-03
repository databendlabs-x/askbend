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

pub struct SnippetFile {
    pub file_path: String,
    pub code_snippets: Vec<String>,
}

pub struct SnippetFiles {
    pub snippet_files: Vec<SnippetFile>,
}

impl SnippetFiles {
    pub fn all_snippets(&self) -> usize {
        self.snippet_files
            .iter()
            .map(|v| v.code_snippets.len())
            .collect::<Vec<usize>>()
            .iter()
            .sum()
    }

    pub fn all_tokens(&self) -> usize {
        self.snippet_files
            .iter()
            .map(|v| {
                v.code_snippets
                    .iter()
                    .map(|v| v.split_whitespace().count())
                    .sum()
            })
            .collect::<Vec<usize>>()
            .iter()
            .sum()
    }
}

pub trait Parse {
    fn parse(path: &str) -> Result<SnippetFile>;
    fn parse_multiple(paths: &[String]) -> Result<SnippetFiles>;
}
