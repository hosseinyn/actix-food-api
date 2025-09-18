use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{NewFood, Food, UpdateFood, establish_connection};
use crate::schema::food;

pub async fn submit_food(food: web::Json<NewFood>) -> impl Responder {
    let mut conn = establish_connection();
    let new_food = food.into_inner();

    match diesel::insert_into(food::table)
        .values(&new_food)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json("Food submitted successfully"),
        Err(err) => {
            eprintln!("DB Insert Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_foods() -> impl Responder {
    let mut conn = establish_connection();
    
    match food::table.select((food::id, food::name, food::quantity, food::price))
        .load::<Food>(&mut conn)
    {
        Ok(foods) => HttpResponse::Ok().json(foods),
        Err(err) => {
            eprintln!("DB Load Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_food(
    food_id: web::Path<i32>,
    food_update: web::Json<UpdateFood>,
) -> impl Responder {
    let mut conn = establish_connection();

    match diesel::update(food::table.find(food_id.into_inner()))
        .set(&food_update.into_inner())
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json("Food updated successfully"),
        Err(err) => {
            eprintln!("DB Update Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_food(food_id: web::Path<i32>) -> impl Responder {
    let mut conn = establish_connection();

    match diesel::delete(food::table.find(food_id.into_inner()))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json("Food deleted successfully"),
        Err(err) => {
            eprintln!("DB Delete Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
