use crate::country::schema::blacklist;
use crate::country::BankData;
use diesel::{prelude::*, sqlite::SqliteConnection};
use std::env;

#[derive(Insertable, Queryable)]
#[table_name = "blacklist"]
pub struct Blacklist {
    iban: String,
    blacklisted: bool,
}

pub fn establish_connection() -> SqliteConnection {
    let db_path = format!(
        "{}/db.sqlite3",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
    );
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path)) // I could return this error to the user, but should I?
}

pub fn blacklist(iban: &str, op: &str) -> Result<(), Box<dyn std::error::Error>> {
    let op = match op {
        "ADD" | "Add" | "add" => Ok(true),
        "REMOVE" | "Remove" | "remove" => Ok(false),
        _ => Err("Failure: Operation not recognized. Use ADD or REMOVE"),
    }?;
    let conn = establish_connection();
    let data = Blacklist {
        iban: iban.to_string(),
        blacklisted: op,
    };
    diesel::replace_into(blacklist::table)
        .values(data)
        .execute(&conn)?;
    Ok(())
}

pub fn is_blacklisted(connection: &SqliteConnection, iban: &str) -> Result<(), String> {
    use blacklist::dsl::blacklist;
    let record = blacklist.find(iban).first::<Blacklist>(connection);
    match record {
        Ok(data) => {
            if data.blacklisted {
                Err(String::from("Failure: IBAN is blacklisted."))
            } else {
                Ok(())
            }
        }
        Err(_) => Ok(()),
    }
}

pub trait Db {
    fn get_bank_data(
        &self,
        connection: &SqliteConnection,
        bank_code: &str,
    ) -> Result<BankData, String>;
    fn fill_table(&self, connection: &SqliteConnection) -> Result<(), Box<dyn std::error::Error>>;
    fn update_table(&self, connection: &SqliteConnection)
        -> Result<(), Box<dyn std::error::Error>>;
}
