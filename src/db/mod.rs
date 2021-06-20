use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};

pub mod schema;
//pub mod models; // we add models directly in country specific files
pub mod de;

pub enum BlacklistOp {
    Add,
    Remove,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbResponse {
    pub success: bool,
    pub message: String,
}

pub fn establish_connection() -> SqliteConnection {
    let db = "./resources/db.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn fill_database(country: String) -> DbResponse {
    let connection = establish_connection();
    match country.as_ref() {
        "DE" | "de" => de::fill_table_request(&connection),
        _ => DbResponse {
            success: false,
            message: String::from("Unsupported country"),
        },
    }
}
pub fn update_database(country: String) -> DbResponse {
    let mut response= download_data(&country);
    if response.success {
        response = fill_database(country);
    }
    response
}
pub fn download_data(country: &String) -> DbResponse {
    match country.as_ref() {
        "DE" | "de" => de::download_data_request(),
        _ => DbResponse {
            success: false,
            message: String::from("Unsupported country"),
        },
    }
}
