use actix_web::{web, Error, HttpResponse, Responder, HttpRequest};
use actix_web::client::{Client};
use actix_http::http::Method;
use std::str;

use crate::app::Config;
use crate::util::jwt::{validate_request};
use crate::url::static_map::StaticMap;

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

pub async fn forward(
    config: web::Data<Config>,
    endpoints: web::Data<StaticMap>,
    client: web::Data<Client>,
    req: HttpRequest,
    body: web::Bytes
) -> impl Responder {
    let x: &Method = req.method();
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