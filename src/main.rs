use actix_web::client::Client;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use moria::{Config, load_endpoints, forward};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = web::Data::new(
        Config::from_file("config.json").unwrap_or_else(|error| panic!("{:?}", error)),
    );

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file(&config.ssl_private_key, SslFiletype::PEM)
        .unwrap();
    ssl_builder
        .set_certificate_chain_file(&config.ssl_public_key)
        .unwrap();

    // OPTIMIZE: The auth map could be compressed into a smaller type than a hash map. This could
    // potentially curb the memory growth of the application - but does not solve the leak - if it
    // still exists.
    let auth_map = web::Data::new(load_endpoints("endpoints.json"));

    let domain = format!("{}:{}", config.ip, config.port);

    HttpServer::new(move || {
        let client = Client::new();

        App::new()
            .app_data(config.clone())
            .app_data(auth_map.clone())
            .data(client)
            .data(web::PayloadConfig::new(config.max_payload_size))
            .default_service(web::route().to(forward))
    })
    .bind_openssl(&domain, ssl_builder)?
    .run()
    .await
}
