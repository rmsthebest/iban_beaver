use crate::country::get_country;
use crate::country::BankData;
use crate::db::{blacklist, establish_connection};
use crate::iban;
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

pub fn verify_request(iban_str: &str) -> IbanResponse {
    let mut iban_response = IbanResponse::new(iban_str);

    let iban = match iban::parse(iban_str) {
        Ok(iban) => {iban_response.valid = true; iban},
        Err(e) => {
            iban_response.message = e;
            return iban_response;
        }
    };

    match iban::verify(iban) {
        Ok(bd) => {
            iban_response.bank_data = Some(bd);
        }
        Err(e) => iban_response.message = e,
    };
    iban_response
}

pub fn fill_table_request(country_code: &str) -> DbResponse {
    let connection = &establish_connection();
    let mut db_response = DbResponse::default();
    match get_country(country_code) {
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


#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn blacklist() {
        let add= blacklist_request("HEJHOPP", "ADD");
        let remove = blacklist_request("HEJHOPP", "REMOVE");
        println!("WARNING: if you see this message, blacklist test failed and your blacklist may be tainted in your test db");
        assert!(add.success);
        assert!(remove.success);
    }

    // dont run fill or update tests by default, they can be slow

    // DE
    #[test]
    #[ignore]
    #[serial]
    fn update_de() {
        assert!(update_table_request("NL").success);
    }
    #[test]
    #[ignore]
    #[serial]
    fn fill_de() {
        assert!(fill_table_request("DE").success);
        assert!(!fill_table_request("DEX").success);
    }

    // NL
    #[test]
    #[ignore]
    #[serial]
    fn update_nl() {
        assert!(update_table_request("NL").success);
    }
    #[test]
    #[ignore]
    #[serial]
    fn fill_nl() {
        assert!(fill_table_request("NL").success);
    }

    // AT
    #[test]
    #[ignore]
    #[serial]
    fn update_at() {
        assert!(update_table_request("AT").success);
    }
    #[test]
    #[ignore]
    #[serial]
    fn fill_at() {
        assert!(fill_table_request("AT").success);
    }

}
