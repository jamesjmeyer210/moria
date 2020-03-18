use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder, HttpRequest};
use std::collections::HashMap;
use actix_web::client::{Client, ClientRequest};
use std::str;

mod model;
mod startup;
mod jwt;

use crate::model::*;
use crate::startup::{load_endpoints, load_config};
use crate::jwt::{validate_request};

// OPTIMIZE: Use streams and iterators for better performance.
// async fn send(client: &Client, origin: &str, req: HttpRequest, body: web::Bytes) -> HttpResponse {

//     let mut forward = client.request(req.method().clone(), format!("{}{}", origin, req.path()));

//     for header in req.headers().iter() {
//          forward = forward.set_header(header.0, header.1.as_bytes());//Box::new(f.set_header(header.0, header.1.as_bytes()));
//     }

//     match forward.send_body(body).await {
//         Ok(mut response) => {
//             let mut response_builder = HttpResponse::build(response.status());
//             for header in response.headers() {
//                 response_builder.set_header(header.0.clone(), header.1.clone());
//             }

//             match response.body().await {
//                 Ok(bytes) => {

//                     response_builder.body(bytes.clone())
//                 },
//                 Err(error) => {
//                     // TODO: log this error and/or return some type of message
//                     println!("{}", error);
//                     response_builder.finish()
//                 }
//             }
//         },
//         Err(error) => {
//             // TODO: log this error and/or return some type of message
//             println!("{}", error);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

async fn send(
    client: &Client,
    url: &str,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {

    // Build the client request for the proxy
    let mut forwarded_req = client
        .request_from(url, req.head())
        .no_decompress();
        
    // Copy the header values from the incoming request to
    // the forwarded request.
    for (header_name, header_value) in req.headers().iter() {
        forwarded_req = forwarded_req.set_header(header_name.clone(), header_value.clone());
    }
    
    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    // Build the response status of the proxy
    let mut client_resp = HttpResponse::build(res.status());
    // Add the response's headers
    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }
    // Return our constructed response
    Ok(client_resp.body(res.body().await?))
}

async fn forward(
    config: web::Data<Config>, 
    endpoints: web::Data<HashMap<String,AuthObj>>,
    client: web::Data<Client>, 
    req: HttpRequest, 
    body: web::Bytes
) -> impl Responder {

    let lookup = format!("{} {}", req.method(), req.path());

    match endpoints.get(lookup.as_str()) {
        Some(endpoint) => {
            match validate_request(&config, &req, &endpoint) {
                Ok(()) => {
                    let url = format!("{}{}", endpoint.origin, req.path());

                    send(&client, &url, req, body).await
                        .unwrap_or_else(|error|{
                            println!("{}", error);
                            HttpResponse::InternalServerError().finish()
                        })
                },
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