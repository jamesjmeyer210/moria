use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::collections::HashMap;
use actix_web::client::{Client, ClientRequest};
use std::str;

mod model;
mod startup;
mod jwt;

use crate::model::*;
use crate::startup::{load_endpoints, load_config};
use crate::jwt::{validate_request};
use actix_http::{ResponseBuilder, ResponseHead};
use std::borrow::{BorrowMut, Borrow};
// use actix_http::http::{HeaderName, HeaderValue};
// use actix_web::http::header::Iter;

use actix_http::http;
use actix_web::http::header;
// fn forward_construct(
//     iter: &mut header::Iter<(http::HeaderName, http::HeaderValue)>,
//     forward: Option<ClientRequest>,
//     header: Option<(http::HeaderName, http::HeaderValue)>
// ) -> Option<ClientRequest>
// {
//     match header {
//         Some(header) => {
//             let f = forward.unwrap().set_header(header.0, (header.1.clone()));
//             let n = iter.next();
//             forward_construct( iter,Some(f), n )
//         },
//         _ => forward,
//     }
// }
//
// fn forward_construct(
//     forward: Option<Box<ClientRequest>>,
//     header: Option<Box<(&http::HeaderName, &http::HeaderValue)>>
// ) -> Option<ClientRequest>
// {
//     match header {
//         // header is of type Box<&(&http::HeaderName, &http::HeaderValue)>
//         Some(header) => {
//             let h:(&http::HeaderName, &http::HeaderValue) = header.as_ref().to_owned();
//
//             Some(forward.unwrap()
//                     .set_header(h.0, h.1.clone()))
//         },
//         _ => None,
//     }
// }
//
// fn init_forward_request(client: &Client, req: &HttpRequest) -> Option<ClientRequest> {
//
//     let method = req.method();
//     let url = format!("{}{}", req.uri().host().unwrap(), req.path());
//
//     let c: Box<ClientRequest> = Box::new(client.request(method.clone(), url));
//
//     let mut x: Option<ClientRequest> = None;
//     req.headers().iter().map(|i| {
//         let h = i.to_owned();
//
//         x = forward_construct(Some(c), Some(Box::new(h)));
//     });
//
//     return x;
// }

// OPTIMIZE: Use streams and iterators for better performance.
async fn send(client: &Client, origin: &str, req: HttpRequest, body: web::Bytes) -> HttpResponse {

    let mut forward = client.request(req.method().clone(), format!("{}{}", origin, req.path()));

    for header in req.headers().iter() {
         forward = forward.set_header(header.0, header.1.as_bytes());//Box::new(f.set_header(header.0, header.1.as_bytes()));
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

    HttpServer::new(move||{
        // OPTIMIZE: Client is created locally because it cannot be safely shared between threads.
        // It could be possible the client could be wrapped in a Mutex and accessed concurrently,
        // but that could lead to thread locks. In the short term we'll have to settle on setting a
        // limit to the max number of threads because each thread owns a client.
        let client = Client::new();
        //let resp_builder = ResponseBuilder::new();

        App::new()
            .app_data(config.clone())
            .app_data(auth_map.clone())
            .data(client)
            .data(web::PayloadConfig::new(config.max_payload_size))
            .default_service(web::route().to(forward))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}