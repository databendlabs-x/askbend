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
use quote::ToTokens;
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
    let mut snippets = Vec::new();

    for item in &ast.items {
        match item {
            syn::Item::Fn(item_fn) => {
                snippets.push(format!("Function: {}", item_fn.sig.ident));
            }
            syn::Item::Struct(item_struct) => {
                snippets.push(format!("Struct: {}", item_struct.ident));
            }
            syn::Item::Enum(item_enum) => {
                snippets.push(format!("Enum: {}", item_enum.ident));
            }
            syn::Item::Mod(item_mod) => {
                snippets.push(format!("Module: {}", item_mod.ident));
            }
            syn::Item::Const(item_const) => {
                snippets.push(format!("Constant: {}", item_const.ident));
            }
            syn::Item::Static(item_static) => {
                snippets.push(format!("Static: {}", item_static.ident));
            }
            syn::Item::Trait(item_trait) => {
                snippets.push(format!("Trait: {}", item_trait.ident));
            }
            syn::Item::Impl(item_impl) => {
                if let Some((_, path, _)) = &item_impl.trait_ {
                    snippets.push(format!(
                        "Impl: {} for {}",
                        path.segments.last().unwrap().ident,
                        item_impl.self_ty.as_ref().to_token_stream()
                    ));
                } else {
                    snippets.push(format!(
                        "Impl: {}",
                        item_impl.self_ty.as_ref().to_token_stream()
                    ));
                }
            }
            syn::Item::Type(item_type) => {
                snippets.push(format!("Type: {}", item_type.ident));
            }
            _ => {}
        }
    }

    snippets
}
