use actix_web::{web, App, HttpServer,};
use actix_web::client::{Client};
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};

mod app;
mod url;
mod util;

use crate::app::{Config};
use crate::url::{StaticMap};
use crate::app::service::{forward};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let config = web::Data::new(
        Config::from_file("config.json")
            .unwrap_or_else(|error|panic!("{:?}", error))
    );

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder.set_private_key_file(&config.ssl_private_key, SslFiletype::PEM).unwrap();
    ssl_builder.set_certificate_chain_file(&config.ssl_public_key).unwrap();

    let host = format!("{}:{}", config.ip, config.port);
    let static_map = StaticMap::from_file("endpoints.json");
    // TODO: also instantiate the dynamic map

    HttpServer::new(move||{

        let client = Client::new();

        App::new()
            .app_data(config.clone())
            .app_data(static_map.clone())
            .data(client)
            .data(web::PayloadConfig::new(config.max_payload_size))
            .default_service(web::route().to(forward))
    })
    .bind_openssl(&host, ssl_builder)?
    .run()
    .await
}