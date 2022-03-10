// Belgium
use super::schema::t_be;
use crate::{country::Country, db::Db};
use calamine::{open_workbook, Reader, Xlsx};
use curl::easy::Easy;
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "t_be"]
pub struct BankData {
    //#[serde(rename = "T_Identification_Number")]
    id: String,
    //#[serde(rename = "T_Institutions_Dutch")]
    name: String,
    //#[serde(rename = "Biccode")]
    bic: String,
}
impl From<BankData> for super::BankData {
    fn from(bank_data: BankData) -> super::BankData {
        super::BankData {
            code: bank_data.id,
            name: bank_data.name.clone(),
            zip: 0,
            city: "".to_string(),
            bic: Some(bank_data.bic),
        }
    }
}
fn create_entry(connection: &SqliteConnection, bank_data: BankData) {
    diesel::insert_into(t_be::table)
        .values(&bank_data)
        .execute(connection)
        .expect("Error inserting new task"); // crash on failure is correct here
}
fn download_data() -> Result<(), curl::Error> {
    let path = format!(
        "{}/be-data-download.xlsx",
        env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
    );
    if let Ok(mut file) = File::create(&path) {
        let mut easy = Easy::new();
        easy.url("https://www.nbb.be/doc/be/be/protocol/r_fulllist_of_codes_current.xlsx")?;
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
pub struct Be {}
impl Db for Be {
    fn get_bank_data(
        &self,
        connection: &SqliteConnection,
        bank_code: &str,
    ) -> Result<super::BankData, String> {
        use super::schema::t_be::dsl::*;
        let data = t_be
            .filter(id.eq(bank_code))
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
            "{}/be-data-download.xlsx",
            env::var("IBAN_BEAVER_RESOURCES").unwrap_or_else(|_| "./resources".into())
        );
        // drop table if it exists already
        diesel::delete(t_be::table).execute(connection).unwrap();

        let mut workbook: Xlsx<_> = open_workbook(path)?;

        let range =
            workbook
                .worksheet_range("Q_FULL_LIST_XLS_REPORT")
                .ok_or(calamine::Error::Msg(
                    "Cannot find xlsx sheet name: 'Q_FULL_LIST_XLS_REPORT'",
                ))??;

        diesel::delete(t_be::table).execute(connection).unwrap();
        let start_row = 2; // Magic number 2, first row has todays date, not headers
        let end_row = range.end().unwrap().0;
        for row in start_row..end_row {
            let id = range.get((row as usize, 0)).unwrap().to_string();
            let bic = range.get((row as usize, 1)).unwrap().to_string();
            let name = range.get((row as usize, 2)).unwrap().to_string();
            let bank_data = BankData { id, bic, name };
            //println!("{:?}", bank_data);
            create_entry(connection, bank_data)
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
impl Country for Be {}
