use crate::country::BankData;
use crate::country::get_country;
use crate::db::{establish_connection, blacklist};
use crate::iban::verify;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IbanResponse {
    iban: String,
    valid: bool,
    bank_data: Option<BankData>,
    message: String,
}
impl IbanResponse {
    fn new(iban: &str) -> IbanResponse {
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

pub fn verify_request(iban: &str) -> IbanResponse {
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

pub fn fill_table_request(iban: &str) -> DbResponse {
    let connection = &establish_connection();
    let mut db_response = DbResponse::default();
    match get_country(iban) {
        Ok(country) => match country.fill_table(connection) {
            Ok(_) => {
                db_response.success = true;
                db_response.message = String::from("Success: table has been (re)filled with data");
            }
            Err(e) => db_response.message = format!("{:?}", e),
        },
        Err(e) => db_response.message = e,
    };
    db_response
}
pub fn update_table_request(iban: &str) -> DbResponse {
    let connection = &establish_connection();
    let mut db_response = DbResponse::default();
    match get_country(iban) {
        Ok(country) => match country.update_table(connection) {
            Ok(_) => {
                db_response.success = true;
                db_response.message = String::from("Success: table has been (re)filled with data");
            }
            Err(e) => db_response.message = format!("{:?}", e),
        },
        Err(e) => db_response.message = e,
    };
    db_response
}
pub fn blacklist_request(iban: &str, op: &str) -> DbResponse {
    let mut db_response = DbResponse::default();
    match blacklist(iban, op) {
        Ok(_) => db_response.success = true,
        Err(e) => db_response.message = format!("{:?}", e),
    };
    db_response
}
