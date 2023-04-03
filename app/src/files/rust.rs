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
use quote::quote;
use syn::File;

use crate::Parse;

pub struct RustCode {
    pub path: String,
    pub snippets: Vec<String>,
}

pub struct RustCodes {
    pub rust_codes: Vec<RustCode>,
}

impl Parse for RustCode {
    type Container = RustCodes;

    fn parse(path: &str) -> Result<Self>
    where Self: Sized {
        let source_code = fs::read_to_string(path)?;
        let ast = parse_rust_code(&source_code)?;
        let snippets = extract_code_snippets(&ast);
        Ok(RustCode {
            path: path.to_string(),
            snippets,
        })
    }

    fn parse_multiple(paths: &[String]) -> Result<Self::Container>
    where Self: Sized {
        let mut rust_codes = Vec::new();

        for path in paths {
            let rust_code = Self::parse(path)?;
            rust_codes.push(rust_code);
        }

        Ok(RustCodes { rust_codes })
    }
}

fn parse_rust_code(source_code: &str) -> Result<File> {
    syn::parse_file(source_code)
        .map_err(|err| anyhow::anyhow!("Failed to parse Rust code: {}", err))
}

fn extract_code_snippets(ast: &File) -> Vec<String> {
    let min_snippet_len = 1024;
    let mut snippets = Vec::new();
    let mut current_snippet = String::new();

    for item in &ast.items {
        let tokens = match item {
            syn::Item::Fn(item_fn) => quote! { #item_fn },
            syn::Item::Struct(item_struct) => quote! { #item_struct },
            syn::Item::Enum(item_enum) => quote! { #item_enum },
            syn::Item::Mod(item_mod) => quote! { #item_mod },
            syn::Item::Const(item_const) => quote! { #item_const },
            syn::Item::Static(item_static) => quote! { #item_static },
            syn::Item::Trait(item_trait) => quote! { #item_trait },
            syn::Item::Impl(item_impl) => quote! { #item_impl },
            syn::Item::Type(item_type) => quote! { #item_type },
            _ => continue,
        };

        let snippet = tokens.to_string();
        current_snippet.push_str(&snippet);

        if current_snippet.as_bytes().len() >= min_snippet_len {
            snippets.push(current_snippet);
            current_snippet = String::new();
        }
    }

    if !current_snippet.is_empty() {
        snippets.push(current_snippet);
    }

    snippets
}
