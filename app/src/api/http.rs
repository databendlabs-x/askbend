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

use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use anyhow::Result;
use log::debug;
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::api::query::query_handler;
use crate::api::status::status_handler;
use crate::configs::ServerConfig;
use crate::DatabendDriver;

#[derive(RustEmbed)]
#[folder = "../web/dist/"]
struct Asset;

pub struct APIHandler {
    pub conf: ServerConfig,
    pub db: DatabendDriver,
}

impl APIHandler {
    pub fn create(conf: &ServerConfig, db: DatabendDriver) -> Self {
        APIHandler {
            conf: conf.clone(),
            db,
        }
    }

    pub async fn start(self) -> Result<()> {
        let conf = self.conf.clone();
        let host = conf.host.clone();
        let port = conf.port;
        let data = self.db.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(data.clone()))
                .route("/status", web::get().to(status_handler))
                .route("/query", web::post().to(query_handler))
                .service(index)
                .service(dist)
        })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await?;

        Ok(())
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    debug!("visit static file: {path}");
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
