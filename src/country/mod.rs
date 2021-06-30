use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::iban::Iban;

pub mod at;
pub mod de;
pub mod nl;
pub mod schema;

#[derive(Serialize, Deserialize, Debug)]
pub struct BankData {
    pub code: String,
    pub name: String,
    pub zip: i32,
    pub city: String,
    pub bic: Option<String>,
}

pub trait Country: Iban + Db {}

pub fn get_country(iban: &str) -> Result<Box<dyn Country>, String> {
    match iban.chars().take(2).collect::<String>().as_ref() {
        "DE" | "De" | "de" => Ok(Box::new(de::De {})),
        "AT" | "At" | "at" => Ok(Box::new(at::At {})),
        "NL" | "Nl" | "nl" => Ok(Box::new(nl::Nl {})),
        _ => Err(String::from("Failure: Country specified is not supported.")),
    }
}

