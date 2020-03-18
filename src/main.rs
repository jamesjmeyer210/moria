use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::client::{Client};
use std::collections::HashMap;
use std::str;

mod model;
mod startup;
mod jwt;

use crate::model::*;
use crate::startup::{load_endpoints, load_config};
use crate::jwt::{validate_request};

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
    // finally, send the request and return any errors if we get them
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

    match endpoints.get(&lookup) {
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
        None => HttpResponse::NotFound().body(lookup)
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let config = web::Data::new(load_config("config.json"));
    // OPTIMIZE: The auth map could be compressed into a smaller type than a hash map. This could
    // potentially curb the memory growth of the application - but does not solve the leak - if it
    // still exists.
    let auth_map= web::Data::new(load_endpoints("endpoints.json"));

    HttpServer::new(move||{

        let client = Client::new();

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