mod schema;
mod models;
mod handlers;
use actix_web::{web, App, HttpServer};
use crate::handlers::{submit_food};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("submit-food" , web::post().to(submit_food))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}