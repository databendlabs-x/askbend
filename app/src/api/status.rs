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
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::DatabendDriver;

#[derive(serde::Serialize)]
struct Response {
    result: String,
}

pub async fn status_handler(db: web::Data<DatabendDriver>) -> impl Responder {
    let response = Response {
        result: format!("{:?}", db.table),
    };
    HttpResponse::Ok().json(response)
}
