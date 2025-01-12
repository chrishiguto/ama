use crate::{routes::question, routes::room};
use actix_cors::Cors;
use actix_web::{middleware, web};
use actix_web::{web::JsonConfig, App, HttpResponse, HttpServer};
use serde_json::json;
use server::RoomServer;
use tokio::{spawn, try_join};

mod database;
mod errors;
mod handler;
mod models;
mod routes;
mod server;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let json_config = JsonConfig::default().error_handler(|err, _req| {
        let error_response = json!({ "message": err.to_string() });

        actix_web::error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(error_response),
        )
        .into()
    });

    let (room_server, server_tx) = RoomServer::new();
    let room_server = spawn(room_server.run());

    let http_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(json_config.clone())
            .configure(room::create_routes)
            .configure(question::create_routes)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    try_join!(http_server, async move { room_server.await.unwrap() })?;

    Ok(())
}
