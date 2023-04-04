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

use std::fs;

use anyhow::Result;
use comrak::format_commonmark;
use comrak::nodes::NodeValue;
use comrak::parse_document;
use comrak::Arena;
use comrak::ComrakOptions;

use crate::Parse;
use crate::SnippetFile;
use crate::SnippetFiles;

pub struct Markdown;

impl Parse for Markdown {
    fn parse(path: &str) -> Result<SnippetFile> {
        let min_section_len = 1024;

        let content = fs::read_to_string(path)?;
        let arena = Arena::new();
        let root = parse_document(&arena, &content, &ComrakOptions::default());

        let mut sections = Vec::new();
        let mut current_section = String::new();

        for node in root.children() {
            match node.data.borrow().value {
                NodeValue::Heading(_) => {
                    if !current_section.is_empty() {
                        sections.push(current_section);
                        current_section = String::new();
                    }
                }
                _ => {
                    let mut section_text = vec![];
                    format_commonmark(node, &ComrakOptions::default(), &mut section_text).unwrap();
                    current_section.push_str(std::str::from_utf8(&section_text).unwrap());
                }
            }
        }

        if !current_section.is_empty() {
            sections.push(current_section);
        }

        // Combine sections with a length less than the minimum length with the previous section
        let mut combined_sections = Vec::new();
        let mut prev_section = String::new();

        for section in sections {
            if (prev_section.len() + section.len()) < min_section_len {
                prev_section.push_str(&section);
            } else {
                if !prev_section.is_empty() {
                    combined_sections.push(prev_section);
                }
                prev_section = section;
            }
        }

        if !prev_section.is_empty() {
            combined_sections.push(prev_section);
        }

        Ok(SnippetFile {
            file_path: path.to_string(),
            code_snippets: combined_sections,
        })
    }

    fn parse_multiple(paths: &[String]) -> Result<SnippetFiles> {
        let mut snippet_files = Vec::new();

        for path in paths {
            let snippet_file = Self::parse(path)?;
            snippet_files.push(snippet_file);
        }

        Ok(SnippetFiles { snippet_files })
    }
}
