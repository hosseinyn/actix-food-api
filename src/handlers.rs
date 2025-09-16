use actix_web::{web, HttpResponse, Responder, Result};
use diesel::prelude::*;

use crate::models::{establish_connection, NewFood};
use crate::schema::Food::dsl::*;

pub async fn submit_food(food: web::Json<NewFood>) -> Result<impl Responder> {
    let mut conn = establish_connection();

    let new_food: NewFood = food.into_inner();

    diesel::insert_into(Food)
        .values(&new_food)
        .execute(&mut conn)
        .expect("failed to submit food");

    Ok(HttpResponse::Ok().json("food submitted successfully"))
}
