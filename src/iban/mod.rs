use crate::db::{self, is_blacklisted};
use crate::db::{BlacklistOp, DbResponse};
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

pub mod de;
pub mod at;

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
}
#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub struct ValidityChecks{
    country: bool,
    length: bool,
    modulo: bool,
    blacklist: bool,
    in_db: bool,
}
impl ValidityChecks {
    fn pre_check_pass(&self) -> bool {
        self.country && self.length && self.modulo
    }
    fn pass(&self) -> bool {
        self.country && self.length && self.modulo && self.blacklist && self.in_db
    }
    fn gen_message(&self) -> String {
        let message;
        if !self.country {
            message = String::from("Failure: Country not supported.");
        } else if !self.length {
            message = String::from("Failure: Length check failed.");
        } else if !self.modulo {
            message = String::from("Failure: Modulo check failed.");
        } else if !self.blacklist {
            message = String::from("Failure: IBAN is blacklisted.");
        } else if !self.in_db {
            message = String::from("Failure: IBAN valid but not found in database.");
        } else {
            message = String::from("");
        }
        message
    }
}
pub fn blacklist_request(iban: String, op: String) -> DbResponse {
    println!("blacklist op requested {}", iban);
    let blacklist_op = match op.as_ref() {
        "add" | "Add" | "ADD" => BlacklistOp::Add,
        "remove" | "Remove" | "REMOVE" => BlacklistOp::Remove,
        _ => {
            return DbResponse {
                success: false,
                message: format!(
                    "Failure. '{}' is an unknown operation. Please use 'add' or 'remove'",
                    op
                ),
            }
        }
    };
    let mut validity_checks = ValidityChecks::default();
    validity_checks.blacklist = true; // We don't care about value, we are changing it in the db.
    validity_checks.in_db = true; // We don't care about this either.
    let country = country_code(&iban);
    if let Some(c) = country {
        match c {
            Country::DE => {
                validity_checks.country = true;
                validity_checks.length = de::verify_length(&iban);
                validity_checks.modulo = verify_mod(&iban);
            },
            _ => (),
        }
    }
    if validity_checks.pre_check_pass() {
        let result = db::blacklist(&iban, blacklist_op);
        match result {
            Ok(_) => DbResponse {success: true, message: String::from("")},
            Err(e) => DbResponse {success: false, message: format!("{:?}", e)}
        }
    } else {
        DbResponse {success: false, message: validity_checks.gen_message()}
    }
}
/// does verification, database lookup and returns json ready response
pub fn verify(iban: &String) -> IbanResponse {
    println!("Verify op requested {}", iban);
    let connection = db::establish_connection();
    //let mut validity_checks = ValidityChecks::new(&connection, iban);
    let mut validity_checks = ValidityChecks::default();
    let country = country_code(iban);
    let mut bank_data = None; // we need bankdata in this scope
    if let Some(c) = country {
        match c {
            Country::DE => {
                validity_checks.country = true;
                validity_checks.length = de::verify_length(iban);
                validity_checks.modulo = verify_mod(iban);
                validity_checks.blacklist = !is_blacklisted(&connection, iban);
                let bank_code = de::bank_code(iban);
                bank_data = db::de::get_bank_data(&connection, bank_code);
                validity_checks.in_db = bank_data.is_some();
            }
            Country::AT => {
                validity_checks.country = true;
                validity_checks.length = at::verify_length(iban);
                validity_checks.modulo = verify_mod(iban);
                validity_checks.blacklist = !is_blacklisted(&connection, iban);
                let bank_code = at::bank_code(iban);
                bank_data = db::at::get_bank_data(&connection, bank_code);
                validity_checks.in_db = bank_data.is_some()
            }
            //_ => (),
        }
    }
    let message = validity_checks.gen_message();
    IbanResponse {
        iban: iban.to_string(),
        valid: validity_checks.pass(),
        bank_data,
        message,
    }


}

// Enum with supported countries
enum Country {
    DE,
    AT,
}

// first two characters are country code
fn country_code(iban: &String) -> Option<Country> {
    match iban.chars().take(2).collect::<String>().as_ref() {
        "DE" | "de" => Some(Country::DE),
        "AT" | "at" => Some(Country::AT),
        _ => None,
    }
}

// Iban modulo check
fn verify_mod(iban: &String) -> bool {
    // first four to the end
    let reorderd_iban = format!(
        "{}{}",
        iban.chars().skip(4).collect::<String>(),
        iban.chars().take(4).collect::<String>()
    );
    if let Ok(integer_iban) = convert_to_int(reorderd_iban) {
        integer_iban.rem_euclid(97) == 1
    } else {
        false
    }
}

fn convert_to_int(reordered_iban: String) -> Result<u128, ParseIntError> {
    let thing = reordered_iban
        .chars()
        .map(|c| char_to_num(c))
        .collect::<String>();
    thing.parse()
}

fn char_to_num(c: char) -> String {
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
