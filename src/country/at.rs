// Austria
use super::schema::t_at;
use super::{Country, Db};
use csv;
use curl::easy::Easy;
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "t_at"]
pub struct BankData {
    #[serde(rename = "Identnummer")]
    id: i32, // Ident numbers arent unique in austria, could remove this field
    #[serde(rename = "Bankleitzahl")]
    code: String, // iban is unique in this db, using as primary key
    #[serde(rename = "Bankenname")]
    name: String,
    #[serde(rename = "PLZ")]
    zip: i32,
    #[serde(rename = "Ort")]
    city: String,
    #[serde(rename = "SWIFT-Code")]
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
    diesel::insert_into(t_at::table)
        .values(&bank_data)
        .execute(connection)
        .expect("Error inserting new task"); // crash on failure is correct here
}
fn download_data() -> Result<(), curl::Error> {
    let path = format!(
        "{}/at-data-download.csv",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
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
pub struct At {}
impl Db for At {
    fn get_bank_data(
        &self,
        connection: &SqliteConnection,
        bank_code: &str,
    ) -> Result<super::BankData, String> {
        use super::schema::t_at::dsl::*;
        let data = t_at
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
        // --- parse csv ---
        let path = format!(
            "{}/at-data-download.csv",
            env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
        );
        // drop table if it exists already
        diesel::delete(t_at::table).execute(connection).unwrap();

        // Fancy footwork to deal with ISO-8859-1 to UTF-8
        let mut file = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
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

    fn update_table(
        &self,
        connection: &SqliteConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        download_data()?;
        self.fill_table(connection)?;
        Ok(())
    }
}
impl Country for At {}
