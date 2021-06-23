//Kennzeichen;Identnummer;Bankleitzahl;Institutsart;Sektor;Firmenbuchnummer;Bankenname;Straße;PLZ;Ort;Postadresse / Straße;Postadresse / PLZ;Postadresse / Ort;Postfach;Bundesland;Telefon;Fax;E-Mail;SWIFT-Code;Homepage;Gründungsdatum
//Mark; identification number; bank code; type of institution; sector; commercial register number; bank name; street; post code; place; postal address / street; postal address / post code; postal address / place; PO box; state; telephone; fax; e-mail; SWIFT code; homepage; Establishment date
use crate::iban;
use super::{schema::*, DbResponse};
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::env;
use curl::easy::Easy;
use std::fs::File;
use std::io::{Read, Write};
use csv;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "t_at"]
pub struct BankData {
    #[serde(rename = "Identnummer")]
    id: i32, // Ident numbers arent unique in austria, could remove this field
    #[serde(rename = "Bankleitzahl")]
    code: i32, // iban is unique in this db, using as primary key
    #[serde(rename = "Bankenname")]
    name: String,
    #[serde(rename = "PLZ")]
    zip: i32,
    #[serde(rename = "Ort")]
    city: String,
    #[serde(rename = "SWIFT-Code")]
    bic: Option<String>,
}
impl From<BankData> for iban::BankData {
    fn from(bank_data: BankData) -> iban::BankData {
        crate::iban::BankData {
            code: bank_data.code,
            name: bank_data.name.clone(),
            zip: bank_data.zip,
            city: bank_data.city.clone(),
            bic: bank_data.bic,
        }
    }
}

fn create_entry(connection: &SqliteConnection, bank_data: BankData) {
    diesel::insert_into(t_at::table)
        .values(&bank_data)
        .execute(connection)
        .expect("Error inserting new task"); // crash on failure is correct here
}

pub fn get_bank_data(connection: &SqliteConnection, iban_bank_code: i32) -> Option<iban::BankData> {
    use super::schema::t_at::dsl::*;
    let data = t_at
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
// This function could be country generic
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
pub fn fill_table(connection: &SqliteConnection) -> Result<(), csv::Error> {
    // --- parse csv ---
    let path = format!(
        "{}/at-data-download.csv",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or("./resources".into())
    );
    // drop table if it exists already
    diesel::delete(t_at::table).execute(connection).unwrap();

    // Fancy footwork to deal with ISO-8859-1 to UTF-8
    let mut file = File::open(path)?;
    let mut buf:Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    // The skip is to skip the garbage above header. Very fragile. Should maybe do it some other way...
    let utf8_csv = buf.iter().map(|&c| c as char).skip(512).collect::<String>();

    let mut rdr = csv::ReaderBuilder::new()
    .delimiter(b';')
    .flexible(true)
    .from_reader(utf8_csv.as_bytes());
    //println!("{:?}", rdr.headers());
    for result in rdr.deserialize() {
        //println!("{:?}", result);
        let bank_data: BankData = result?;
        create_entry(connection, bank_data);
    }

    Ok(())
}
// This one should probably be generic
pub fn download_data_request() -> DbResponse {
    match download_data() {
        Ok(_) => DbResponse {
            success: true,
            message: format!("Downloaded database for Austria (at)"),
        },
        Err(e) => DbResponse {
            success: false,
            message: format!("Error: {:?}", e),
        },
    }
}
// still de
pub fn download_data() -> Result<(), curl::Error> {
    let path = format!(
        "{}/at-data-download.csv",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or("./resources".into())
    );
    if let Ok(mut file) = File::create(&path) {
        let mut easy = Easy::new();
        easy.url("https://www.oenb.at/docroot/downloads_observ/sepa-zv-vz_gesamt.csv")?;
        easy.follow_location(true)?;
        easy.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
        easy.perform()?;
    }

    Ok(())
}
