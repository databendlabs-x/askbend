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

use regex::Regex;

pub fn escape_sql_string(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('\'', "''")
        .replace('\n', " ")
        .replace('\r', "\\r")
}

pub fn remove_markdown_links(input: &str) -> String {
    let link_regex = Regex::new(r"\[(?P<text>[^\]]+)\]\((?P<url>[^\)]+)\)").unwrap();
    let result = link_regex.replace_all(input, "$text");
    result.to_string()
}

pub fn replace_multiple_spaces(input: &str) -> String {
    let re = Regex::new(r" +").unwrap();
    re.replace_all(input, " ").to_string()
}

pub trait LengthWithoutSymbols {
    fn length_without_symbols(&self) -> usize;
}

impl LengthWithoutSymbols for String {
    fn length_without_symbols(&self) -> usize {
        self.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .count()
    }
}
