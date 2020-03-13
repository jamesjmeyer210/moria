use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::collections::HashMap;
use actix_web::client::{Client};
use std::str;

const PAYLOAD_SIZE: usize = 200_000;

mod model;
mod startup;
mod jwt;

use crate::model::*;
use crate::startup::{load_endpoints, load_config};
use crate::jwt::{validate_request};

// OPTIMIZE: Use streams and iterators for better performance.
async fn send(client: &Client, origin: &str, req: HttpRequest, body: web::Bytes) -> HttpResponse {

    let mut forward = client.request(req.method().clone(), format!("{}{}", origin, req.path()));

    for header in req.headers().iter() {
        forward = forward.set_header(header.0, header.1.as_bytes());
    }

    match forward.send_body(body).await {
        Ok(mut response) => {
            let mut response_builder = HttpResponse::build(response.status());
            for header in response.headers() {
                response_builder.set_header(header.0.clone(), header.1.clone());
            }

            match response.body().await {
                Ok(bytes) => {
                    response_builder.body(bytes.clone())
                },
                Err(error) => {
                    // TODO: log this error and/or return some type of message
                    println!("{}", error);
                    response_builder.finish()
                }
            }
        },
        Err(error) => {
            // TODO: log this error and/or return some type of message
            println!("{}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn forward(config: web::Data<Config>, endpoints: web::Data<HashMap<String,AuthObj>>,
    client: web::Data<Client>, req: HttpRequest, body: web::Bytes) -> impl Responder {

    let lookup = format!("{} {}", req.method(), req.path());

    match endpoints.get(lookup.as_str()) {
        Some(endpoint) => {
            match validate_request(&config, &req, &endpoint) {
                Ok(()) => send(client.get_ref(), endpoint.origin.as_str(), req, body).await,
                Err(error) => {
                    println!("{} {:?}", lookup, error);
                    HttpResponse::Unauthorized().finish()
                },
            }
        },
        None => HttpResponse::NotFound().body(format!("{} {}", req.method(), req.path()))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let config = web::Data::new(load_config("config.json"));
    let auth_map= web::Data::new(load_endpoints("endpoints.json"));

    // let config = load_config("config.json");
    // let auth_map= load_endpoints("endpoints.json");

    HttpServer::new(move||{

        let client = Client::new();

        App::new()
            .app_data(config.clone())
            .app_data(auth_map.clone())
            .data(client)
            .data(web::PayloadConfig::new(PAYLOAD_SIZE))
            .default_service(web::route().to(forward))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}