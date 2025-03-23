mod config;
mod handlers;
mod models;
mod routers;
mod services;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use handlers::user::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service at port 8080...");
    let app = web::Data::new(AppState {
        users: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app.clone())
            .service(routers::user::user_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
