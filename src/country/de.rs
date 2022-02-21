// Germany
use super::schema::t_de;
use crate::{country::Country, country::Iban, db::Db};
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use curl::easy::Easy;
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "t_de"]
pub struct BankData {
    #[serde(rename = "Datensatz-nummer")]
    id: i32,
    #[serde(rename = "Bank-leitzahl")]
    code: String,
    #[serde(rename = "Kurzbezeichnung")]
    name: String,
    #[serde(rename = "PLZ")]
    zip: i32,
    #[serde(rename = "Ort")]
    city: String,
    #[serde(rename = "BIC")]
    bic: Option<String>,
}
impl From<BankData> for super::BankData {
    fn from(bank_data: BankData) -> super::BankData {
        super::BankData {
            code: bank_data.code,
            name: bank_data.name.clone(),
            zip: bank_data.zip,
            city: bank_data.city.clone(),
            bic: bank_data.bic,
        }
    }
}
fn create_entry(connection: &SqliteConnection, bank_data: BankData) {
    diesel::insert_into(t_de::table)
        .values(&bank_data)
        .execute(connection)
        .expect("Error inserting new task"); // crash on failure is correct here
}
fn download_data() -> Result<(), curl::Error> {
    let path = format!(
        "{}/de-data-download.xlsx",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
    );
    if let Ok(mut file) = File::create(&path) {
        let mut easy = Easy::new();
        easy.url("https://www.bundesbank.de/resource/blob/602630/38698577eac2fb9d6fe2265bbbeacdd5/mL/blz-aktuell-xls-data.xlsx")?;
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
pub struct De {}
impl Db for De {
    fn get_bank_data(
        &self,
        connection: &SqliteConnection,
        bank_code: &str,
    ) -> Result<super::BankData, String> {
        use super::schema::t_de::dsl::*;
        let data = t_de
            .filter(code.eq(bank_code))
            .limit(1)
            .load::<BankData>(connection)
            .expect("Error loading posts")
            .pop();

        match data {
            Some(d) => Ok(super::BankData::from(d)),
            None => Err(String::from("")),
        }
    }

    fn fill_table(&self, connection: &SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
        // --- parse xml ---

        let path = format!(
            "{}/de-data-download.xlsx",
            env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
        );
        // drop table if it exists already
        diesel::delete(t_de::table).execute(connection).unwrap();

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

    fn update_table(
        &self,
        connection: &SqliteConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        download_data()?;
        self.fill_table(connection)?;
        Ok(())
    }
}
impl Iban for De {
    fn verify_length(&self, iban: &str) -> Result<(), String> {
        let nof_chars = iban.chars().count();
        if nof_chars == 22 {
            Ok(())
        } else {
            Err(String::from("Failure: Invalid length of IBAN for country"))
        }
    }

    fn bank_code(&self, iban: &str) -> String {
        iban.chars()
            .skip(4)
            .take(8)
            .collect::<String>()
    }
}
impl Country for De {}
