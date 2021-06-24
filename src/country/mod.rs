use diesel::{prelude::*, sqlite::SqliteConnection};
use schema::blacklist;
use serde::{Deserialize, Serialize};
use std::env;

pub mod at;
pub mod de;
pub mod schema;

#[derive(Serialize, Deserialize, Debug)]
pub struct BankData {
    pub code: i32,
    pub name: String,
    pub zip: i32,
    pub city: String,
    pub bic: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IbanResponse {
    iban: String,
    valid: bool,
    bank_data: Option<BankData>,
    message: String,
}
impl IbanResponse {
    fn new(iban: &String) -> IbanResponse {
        IbanResponse {
            iban: iban.to_string(),
            valid: false,
            bank_data: None,
            message: "".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DbResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "blacklist"]
pub struct Blacklist {
    iban: String,
    blacklisted: bool,
}

fn establish_connection() -> SqliteConnection {
    let db_path = format!(
        "{}/db.sqlite3",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or("./resources".into())
    );
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path)) // I could return this error to the user, but should I?
}

fn get_country(iban: &String) -> Result<Box<dyn Country>, String> {
    match iban.chars().take(2).collect::<String>().as_ref() {
        "DE" | "De" | "de" => Ok(Box::new(de::De {})),
        "AT" | "At" | "at" => Ok(Box::new(at::At {})),
        _ => Err(String::from("Failure: Country specified is not supported.")),
    }
}

pub fn verify_request(iban: &String) -> IbanResponse {
    let mut iban_response = IbanResponse::new(iban);
    match verify(iban) {
        Ok(bd) => {
            iban_response.valid = true;
            iban_response.bank_data = Some(bd);
        }
        Err(e) => iban_response.message = e,
    };
    iban_response
}

pub fn fill_table_request(iban: &String) -> DbResponse {
    let connection = &establish_connection();
    let mut db_response = DbResponse::default();
    match get_country(iban) {
        Ok(country) => match country.fill_table(connection) {
            Ok(_) => {
                db_response.success = true;
                db_response.message = format!("Success: table has been (re)filled with data");
            }
            Err(e) => db_response.message = format!("{:?}", e),
        },
        Err(e) => db_response.message = e,
    };
    db_response
}
pub fn update_table_request(iban: &String) -> DbResponse {
    let connection = &establish_connection();
    let mut db_response = DbResponse::default();
    match get_country(iban) {
        Ok(country) => match country.update_table(connection) {
            Ok(_) => {
                db_response.success = true;
                db_response.message = format!("Success: table has been (re)filled with data");
            }
            Err(e) => db_response.message = format!("{:?}", e),
        },
        Err(e) => db_response.message = e,
    };
    db_response
}
pub fn blacklist_request(iban: &String, op: &String) -> DbResponse {
    let mut db_response = DbResponse::default();
    match blacklist(iban, op) {
        Ok(_) => db_response.success = true,
        Err(e) => db_response.message = format!("{:?}", e),
    };
    db_response
}

fn verify(iban: &String) -> Result<BankData, String> {
    let country = get_country(iban)?;
    country.verify_length(iban)?;
    country.verify_mod(iban)?;
    let bank_code = country.bank_code(iban);
    let connection = &establish_connection();
    is_blacklisted(connection, iban)?;
    let bank_data = country.get_bank_data(connection, bank_code)?;

    Ok(bank_data)
}

fn blacklist(iban: &String, op: &String) -> Result<(), Box<dyn std::error::Error>> {
    let op = match op.as_ref() {
        "ADD" | "Add" | "add" => Ok(true),
        "REMOVE" | "Remove" | "remove" => Ok(false),
        _ => Err("Failure: Operation not recognized. Use ADD or REMOVE"),
    }?;
    let conn = establish_connection();
    let data = Blacklist {
        iban: iban.to_string(),
        blacklisted: op.into(),
    };
    diesel::replace_into(schema::blacklist::table)
        .values(data)
        .execute(&conn)?;
    Ok(())
}
fn is_blacklisted(connection: &SqliteConnection, iban: &String) -> Result<(), String> {
    use schema::blacklist::dsl::blacklist;
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

trait Country: Iban + Db {}

trait Iban {
    // Each country needs to implement these
    fn bank_code(&self, iban: &String) -> i32;
    fn verify_length(&self, iban: &String) -> Result<(), String>;

    // These all have generics that work for at least some countires (replace if needed)
    fn verify_mod(&self, iban: &String) -> Result<(), String> {
        // first four to the end
        let reorderd_iban = format!(
            "{}{}",
            iban.chars().skip(4).collect::<String>(),
            iban.chars().take(4).collect::<String>()
        );
        if let Ok(integer_iban) = self.convert_to_int(reorderd_iban) {
            if integer_iban.rem_euclid(97) == 1 {
                Ok(())
            } else {
                Err("Failure: IBAN modulo is not 1".to_string())
            }
        } else {
            Err("Failure: IBAN has invalid characters inside".to_string())
        }
    }
    fn convert_to_int(&self, reordered_iban: String) -> Result<u128, std::num::ParseIntError> {
        let thing = reordered_iban
            .chars()
            .map(|c| self.char_to_num(c))
            .collect::<String>();
        thing.parse()
    }
    fn char_to_num(&self, c: char) -> String {
        match c {
            'a' | 'A' => "10".to_string(),
            'b' | 'B' => "11".to_string(),
            'c' | 'C' => "12".to_string(),
            'd' | 'D' => "13".to_string(),
            'e' | 'E' => "14".to_string(),
            'f' | 'F' => "15".to_string(),
            'g' | 'G' => "16".to_string(),
            'h' | 'H' => "17".to_string(),
            'i' | 'I' => "18".to_string(),
            'j' | 'J' => "19".to_string(),
            'k' | 'K' => "20".to_string(),
            'l' | 'L' => "21".to_string(),
            'm' | 'M' => "22".to_string(),
            'n' | 'N' => "23".to_string(),
            'o' | 'O' => "24".to_string(),
            'p' | 'P' => "25".to_string(),
            'q' | 'Q' => "26".to_string(),
            'r' | 'R' => "27".to_string(),
            's' | 'S' => "28".to_string(),
            't' | 'T' => "29".to_string(),
            'u' | 'U' => "30".to_string(),
            'v' | 'V' => "31".to_string(),
            'w' | 'W' => "32".to_string(),
            'x' | 'X' => "33".to_string(),
            'y' | 'Y' => "34".to_string(),
            'z' | 'Z' => "35".to_string(),
            _ => c.to_string(),
        }
    }
}
trait Db {
    fn get_bank_data(
        &self,
        connection: &SqliteConnection,
        iban_bank_code: i32,
    ) -> Result<BankData, String>;
    fn fill_table(&self, connection: &SqliteConnection) -> Result<(), Box<dyn std::error::Error>>;
    fn update_table(&self, connection: &SqliteConnection)
        -> Result<(), Box<dyn std::error::Error>>;
}
