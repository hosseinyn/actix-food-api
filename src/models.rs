use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::schema::food;
use std::env;
use dotenv::dotenv;

#[derive(Queryable, Serialize)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = food)]
pub struct NewFood {
    pub name: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = food)]
pub struct UpdateFood {
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub price: Option<i32>,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}