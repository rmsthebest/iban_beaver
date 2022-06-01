// Netherlands
/*
NLkk bbbb cccc cccc cc
b = BIC Bank code
c = Account number
*/
use super::schema::t_nl;
use crate::{country::Country, db::Db};
use calamine::{open_workbook, Reader, Xlsx};
use curl::easy::Easy;
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "t_nl"]
pub struct BankData {
    //#[serde(alias = "Identifier")]
    code: String,
    //#[serde(alias = "Naam betaaldienstverlener")]
    name: String,
    //#[serde(alias = "BIC")]
    bic: String,
}
impl From<BankData> for super::BankData {
    fn from(bank_data: BankData) -> super::BankData {
        super::BankData {
            code: bank_data.code,
            name: bank_data.name,
            zip: 0,
            city: String::new(),
            bic: Some(bank_data.bic),
        }
    }
}
fn create_entry(connection: &SqliteConnection, bank_data: Vec<BankData>) {
    diesel::insert_into(t_nl::table)
        .values(&bank_data)
        .execute(connection)
        .expect("Error inserting new task"); // crash on failure is correct here
}
fn download_data() -> Result<(), curl::Error> {
    let path = format!(
        "{}/nl-data-download.xlsx",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
    );
    if let Ok(mut file) = File::create(&path) {
        let mut easy = Easy::new();
        easy.url("https://www.betaalvereniging.nl/wp-content/uploads/BIC-lijst-NL.xlsx")?;
        easy.useragent(
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:60.0) Gecko/20100101 Firefox/60.0'",
        )?;
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
pub struct Nl {}
impl Db for Nl {
    fn get_bank_data(
        &self,
        connection: &SqliteConnection,
        bank_code: &str,
    ) -> Result<super::BankData, String> {
        use super::schema::t_nl::dsl::*;
        let data = t_nl
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
        //use calamine::{Range, DataType};
        // --- parse xml ---

        let path = format!(
            "{}/nl-data-download.xlsx",
            env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
        );

        let mut workbook: Xlsx<_> = open_workbook(path)?;

        let range = workbook
            .worksheet_range("BIC-lijst")
            .ok_or(calamine::Error::Msg("Cannot find sheet: 'BIC-lijst'"))??;

        diesel::delete(t_nl::table).execute(connection).unwrap();
        let start_row = 4; // headers are on row 4
        let end_row = range.end().unwrap().0;
        let mut bank_data = Vec::new();
        for row in start_row..end_row {
            let bic = range.get((row as usize, 0)).unwrap().to_string();
            let code = range.get((row as usize, 1)).unwrap().to_string();
            let name = range.get((row as usize, 2)).unwrap().to_string();
            bank_data.push(BankData { code, name, bic });
        }
        create_entry(connection, bank_data);

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
impl Country for Nl {}
