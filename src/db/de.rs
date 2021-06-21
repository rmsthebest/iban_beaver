use super::{schema::*, DbResponse};
use crate::db;
use crate::iban;
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use diesel::update;
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::fs::File;
use std::env;
use curl::easy::Easy;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "t_de"]
pub struct BankData {
    #[serde(rename = "Datensatz-nummer")]
    id: i32,
    #[serde(rename = "Bankleitzahl")]
    code: i32,
    #[serde(rename = "Kurzbezeichnung")]
    name: String,
    #[serde(rename = "PLZ")]
    zip: i32,
    #[serde(rename = "Ort")]
    city: String,
    #[serde(rename = "BIC")]
    bic: Option<String>,
    #[serde(default)]
    blacklisted: bool,
}
impl From<BankData> for iban::BankData {
    fn from(bank_data: BankData) -> iban::BankData {
        crate::iban::BankData {
            code: bank_data.code,
            name: bank_data.name.clone(),
            zip: bank_data.zip,
            city: bank_data.city.clone(),
            bic: bank_data.bic,
            blacklisted: bank_data.blacklisted,
        }
    }
}

fn create_entry(connection: &SqliteConnection, bank_data: BankData) {
    diesel::insert_into(t_de::table)
        .values(&bank_data)
        .execute(connection)
        .expect("Error inserting new task"); // crash on failure is correct here
}

pub fn get_bank_data(connection: &SqliteConnection, iban_bank_code: i32) -> Option<iban::BankData> {
    use super::schema::t_de::dsl::*;
    let data = t_de
        .filter(code.eq(iban_bank_code))
        .limit(1)
        .load::<BankData>(connection)
        .expect("Error loading posts")
        .pop();

    match data {
        Some(d) => Some(iban::BankData::from(d)),
        None => None,
    }
}

pub fn fill_table_request(connection: &SqliteConnection) -> DbResponse {
    match fill_table(connection) {
        Ok(()) => DbResponse {
            success: true,
            message: format!("Success: table has been (re)filled with data"),
        },
        Err(e) => DbResponse {
            success: false,
            message: format!("{:?}", e),
        },
    }
}
pub fn fill_table(connection: &SqliteConnection) -> Result<(), calamine::Error> {
    // --- parse xml ---

    let path = format!("{}/de-data-download.xlsx", env::var("IBAN_BEAVER_RESOURCES").unwrap_or("./resources".into()));

    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Daten")
        .ok_or(calamine::Error::Msg("Cannot find 'Daten'"))??;
    let iter = RangeDeserializerBuilder::new().from_range(&range)?;

    // put in db
    for result in iter {
        let bank_data: BankData = result?;
        create_entry(connection, bank_data);
    }
    Ok(())
}

pub fn blacklist(connection: &SqliteConnection, bank_code: i32, op: db::BlacklistOp) -> DbResponse {
    use super::schema::t_de::dsl::*;
    let new_value= match op {
        db::BlacklistOp::Add => true,
        db::BlacklistOp::Remove => false,
    }; 
    // UPDATE t_de SET blacklisted = REPLACE(blacklisted, false, true) WHERE code = 10077777;
    let result = update(t_de.filter(code.eq_all(bank_code))).set(blacklisted.eq_all(new_value)).execute(connection);

    match result {
        Ok(_) => DbResponse {
            success: true,
            message: format!("Success: Blacklisted banks with bank code {}", bank_code),
        },
        Err(e) => DbResponse {
            success: false,
            message: format!("Error: {:?}", e),
        },
    }
}

pub fn download_data_request() -> DbResponse {
    match download_data() {
        Ok(_) => DbResponse {success: true, message: format!("Downloaded database for Germany (de)")},
        Err(e) => DbResponse {success: false, message: format!("Error: {:?}", e)},
    }
}
pub fn download_data() -> Result<(), curl::Error> {
    let path = format!("{}/de-data-download.xlsx", env::var("IBAN_BEAVER_RESOURCES").unwrap_or("./resources".into()));
    if let Ok(mut file) = File::create(&path) {
        let mut easy = Easy::new();
        easy.url("https://www.bundesbank.de/resource/blob/602630/38698577eac2fb9d6fe2265bbbeacdd5/mL/blz-aktuell-xls-data.xlsx")?;
        easy.follow_location(true)?;
        easy.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        easy.perform()?;    
    }

    Ok(())
}
