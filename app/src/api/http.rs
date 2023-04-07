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
use actix_web::HttpServer;
use anyhow::Result;

use crate::api::query::query_handler;
use crate::api::status::status_handler;
use crate::configs::ServerConfig;
use crate::DatabendDriver;

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
                .service(actix_files::Files::new("/", "./web/dist/").index_file("index.html"))
        })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await?;

        Ok(())
    }
}
