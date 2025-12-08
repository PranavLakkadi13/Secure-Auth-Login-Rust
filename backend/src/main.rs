use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use env_logger::Env;

use crate::handler::{login_user, profile, register};

mod auth;
mod db_service;
mod handler;
mod model;
mod password_helper;
mod schema;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(register)
            .service(login_user)
            .service(profile),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db_service::Dbservice::new();
    let app_data = web::Data::new(db);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("The server running....");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(init)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_method()
                    .allow_any_origin()
                    .allow_any_header()
                    .supports_credentials(),
            )
    })
    .bind("0.0.0.0:9000")
    .unwrap()
    .run()
    .await

}
