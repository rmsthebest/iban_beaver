use crate::db::{DbResponse, BlacklistOp};
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

use super::db;

#[derive(Serialize, Deserialize, Debug)]
pub struct BankData {
    pub code: i32,
    pub name: String,
    pub zip: i32,
    pub city: String,
    pub bic: Option<String>,
    pub blacklisted: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IbanResponse {
    valid: bool,
    message: String,
    iban: String,
    bank_data: Option<BankData>,
}
impl IbanResponse {
    fn new(message: String, iban: String, bank_data: Option<BankData>) -> IbanResponse {
        let valid = match &bank_data {
            Some(bd) => !bd.blacklisted,
            None => false,
        };
        IbanResponse {
            valid,
            message,
            iban,
            bank_data,
        }
    }
}
pub fn blacklist_request(iban: &String, op: &String) -> DbResponse {
    let blacklist_op = match op.as_ref() {
        "add" | "Add" | "ADD" => BlacklistOp::Add, 
        "remove" | "Remove" | "REMOVE" => BlacklistOp::Remove, 
        _ => return DbResponse {success: false, message: format!("Failure. '{}' is an unknown operation. Please use 'add' or 'remove'", op)},
    };
    let primary_valid = verify_length(iban) && verify_mod(iban); // if we want more specific error messages we can do that here
    if primary_valid {
        let connection = db::establish_connection();
        let country_code = country_code(iban);
        let bank_code = bank_code(iban);
        match country_code.as_ref() {
            "DE" => db::de::blacklist(&connection, bank_code, blacklist_op),
            _ => DbResponse {
                success: false,
                message: format!("Failure: Missing blacklist feature for {}", country_code),
            },
        }
    } else {
        DbResponse {
            success: false,
            message: format!("Failure: Invalid iban: {}", iban),
        }
    }
}
/// does verification, database lookup and returns json ready response
pub fn verify(iban: &String) -> IbanResponse {
    let primary_valid = verify_length(iban) && verify_mod(iban); // if we want more specific error messages we can do that here
    if primary_valid {
        let connection = db::establish_connection();
        let country_code = country_code(iban);
        let bank_code = bank_code(iban);
        match country_code.as_ref() {
            "DE" => IbanResponse::new(
                "".to_string(),
                iban.to_string(),
                db::de::get_bank_data(&connection, bank_code),
            ),
            _ => panic!("missing db get impl for valid country"),
        }
    } else {
        IbanResponse {
            valid: false,
            message: "Failed primary checks: Invalid length, modulo, or unsupported country"
                .to_string(),
            iban: "".to_string(),
            bank_data: None,
        }
    }
}

// first two characters are country code
fn country_code(iban: &String) -> String {
    iban.chars().take(2).collect::<String>()
}
fn bank_code(iban: &String) -> i32 {
    iban.chars()
        .skip(4)
        .take(8)
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

// Countries have different valid lengths of iban
fn verify_length(iban: &String) -> bool {
    let nof_chars = iban.chars().count();
    let country_code = country_code(iban);
    let correct_nof_chars = match country_code.as_ref() {
        "DE" => 22,
        _ => return false,
    };
    nof_chars == correct_nof_chars
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
    println!("{}", thing);
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
