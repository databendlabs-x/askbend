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

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use log::error;

use crate::Config;
use crate::QALLM;

#[derive(serde::Deserialize)]
pub struct QAQuery {
    query: String,
}

#[derive(serde::Serialize)]
struct Response {
    result: String,
}

/// curl -X POST -H "Content-Type: application/json" -d '{"query": "whats the fast way to load data to databend"}' http://localhost:8081/query
pub async fn qa_query_handler(
    query: web::Json<QAQuery>,
    conf: web::Data<Config>,
) -> impl Responder {
    let llm = QALLM::create(&conf);
    let result = llm.query(&query.query).await;
    match result {
        Ok(result) => {
            let response = if !result.is_empty() {
                Response { result }
            } else {
                Response {
                    result: "Sorry, I dont know how to help with that.".to_string(),
                }
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            error!("query handler error:{:?}", e);
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("{:?}", e))
        }
    }
}
