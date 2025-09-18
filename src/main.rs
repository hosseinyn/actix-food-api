mod schema;
mod models;
mod handlers;
use actix_web::{web, App, HttpServer};
use crate::handlers::{submit_food , get_foods , update_food , delete_food};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/submit-food" , web::post().to(submit_food))
            .route("/get-foods" , web::post().to(get_foods))
            .route("/update-food/{food_id}" , web::patch().to(update_food))
            .route("/delete-food/{food_id}" , web::delete().to(delete_food))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}