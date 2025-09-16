use diesel::prelude::*;
use serde::{Deserialize , Serialize};
use diesel::sqlite::SqliteConnection;
use crate::schema::Food;
use std::env;
use dotenv::dotenv;

#[derive(Queryable , Serialize)]
pub struct FoodStruct {
    pub id : i32,
    pub name : String,
    pub quantity : i32,
    pub price : i32
}

#[derive(Queryable , Insertable , Serialize , Deserialize)]
#[diesel(table_name=Food)]
pub struct NewFood {
    pub name : String,
    pub quantity : i32,
    pub price : i32
}

#[derive(Deserialize , AsChangeset)]
#[diesel(table_name=Food)]
pub struct UpdateFood {
    pub name : Option<String>,
    pub quantity : Option<i32>,
    pub price : Option<i32>
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect(
                  "DATABASE_URL did not set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!
        ("Error while connection: {}",  
        database_url))
}