// Netherlands
/*
NLkk bbbb cccc cccc cc
b = BIC Bank code
c = Account number 
*/
use super::schema::t_nl;
use crate::{country::Country, country::Iban, db::Db};
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
fn create_entry(connection: &SqliteConnection, bank_data: BankData) {
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
        easy.useragent("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:60.0) Gecko/20100101 Firefox/60.0'")?;
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

        let range= workbook
            .worksheet_range("BIC-lijst")
            .ok_or(calamine::Error::Msg("Cannot find 'BIC-lijst'"))??
            ;
        
        diesel::delete(t_nl::table).execute(connection).unwrap();
        let start_row = 4; // Magic number 4 because parsing this thing sucks for some reason
        let end_row = range.end().unwrap().0;
        for row in start_row..end_row {
            let bic = range.get((row as usize,0)).unwrap().to_string();
            let code = range.get((row as usize,1)).unwrap().to_string();
            let name = range.get((row as usize,2)).unwrap().to_string();
            let bank_data = BankData {
                code,
                name,
                bic
            };
            //println!("{:?}", bank_data);
            create_entry(connection, bank_data)
        }

        


        //let range: calamine::Range<calamine::DataType> = calamine::Range::new((3,0), range_end.end().unwrap());
       /* 
        let mut iter = RangeDeserializerBuilder::with_headers(&["BIC", "Identifier","Naam betaaldienstverlener"])
        //let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;
        
        // skip all garbage above real headers, and start after real headers
        while let Some(result) = iter.next() {
            println!("{:?}", result);
            let res: Result<BankData, DeError> = result;
            if let Ok(bd) = res{
                if &bd.name == "Naam betaaldienstverlener" 
                || &bd.bic == "BIC" 
                || &bd.code == "Identifier" 
                {
                    break;
                }
            }
        }
        */
        //println!("{:?}", range);

        // drop table if it exists already
        //diesel::delete(t_nl::table).execute(connection).unwrap();
        // put in db
        /*
        for result in iter {
            println!("{:?}", result);
            let bank_data: BankData = result?;
            println!("{:?}", bank_data);
            create_entry(connection, bank_data);
        }
        */
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
impl Iban for Nl {
    fn verify_length(&self, iban: &str) -> Result<(), String> {
        let expected_length = 18;
        let length = iban.chars().count();
        if length == expected_length {
            Ok(())
        } else {
            Err(format!("Failure: Invalid length of IBAN for country ({}), expected {}", length, expected_length))
        }
    }

    fn bank_code(&self, iban: &str) -> String {
        iban.chars()
            .skip(4)
            .take(4)
            .collect::<String>()
    }
}
impl Country for Nl {}
