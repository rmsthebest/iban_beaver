use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::env;
use schema::blacklist;

pub mod schema;
//pub mod models; // we add models directly in country specific files
pub mod at;
pub mod de;

#[derive(Insertable, Queryable)]
#[table_name = "blacklist"]
pub struct Blacklist {
    iban: String,
    blacklisted: bool,
}

#[derive(Copy,Clone)]
pub enum BlacklistOp {
    Add,
    Remove,
}
impl From<BlacklistOp> for bool {
    fn from(op: BlacklistOp) -> bool {
        match op {
            BlacklistOp::Add => true,
            BlacklistOp::Remove => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbResponse {
    pub success: bool,
    pub message: String,
}

pub fn establish_connection() -> SqliteConnection {
    let db_path = format!(
        "{}/db.sqlite3",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or("./resources".into())
    );
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

pub fn fill_database(country: String) -> DbResponse {
    println!("fill database started for: {} ", country);
    let connection = establish_connection();
    match country.as_ref() {
        "DE" | "de" => de::fill_table_request(&connection),
        "AT" | "at" => at::fill_table_request(&connection),
        _ => DbResponse {
            success: false,
            message: String::from("Unsupported country"),
        },
    }
}
pub fn update_database(country: String) -> DbResponse {
    println!("update database started for: {} ", country);
    let mut response = download_data(&country);
    if response.success {
        response = fill_database(country);
    }
    response
}
pub fn download_data(country: &String) -> DbResponse {
    println!("Download data started for: {} ", country);
    match country.as_ref() {
        "DE" | "de" => de::download_data_request(),
        "AT" | "at" => at::download_data_request(),
        _ => DbResponse {
            success: false,
            message: String::from("Unsupported country"),
        },
    }
}
pub fn blacklist(iban: &String, op: BlacklistOp) -> QueryResult<usize> {
    let conn = establish_connection();
    let data = Blacklist{iban: iban.to_string(), blacklisted: op.into()};
    diesel::insert_into(schema::blacklist::table).values(data).execute(&conn)
}
pub fn is_blacklisted(connection: &SqliteConnection, iban: &String) -> bool {
    use schema::blacklist::dsl::blacklist;
    let record = blacklist.find(iban).first::<Blacklist>(connection);
    match record {
        Ok(data) => data.blacklisted,
        Err(_) => false,
    }


}
