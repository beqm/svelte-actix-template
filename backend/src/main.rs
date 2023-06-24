use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use log::info;

pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let host = "127.0.0.1";
    let port = 8080;

    info!("Running on http://{}:{}", host, port);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(api::routes())
    })
    .bind((host, port))?
    .run()
    .await
}
