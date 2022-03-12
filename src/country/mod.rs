use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::db::Db;
//use crate::iban::Iban;

pub mod at;
pub mod be;
pub mod de;
pub mod nl;
pub mod schema;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct BankData {
    pub code: String,
    pub name: String,
    pub zip: i32,
    pub city: String,
    pub bic: Option<String>,
}

//pub trait Country: Iban + Db {}
pub trait Country: Db {}

pub fn get_country(country_code: &str) -> Result<Box<dyn Country>, String> {
    match country_code {
        "AT" | "At" | "at" => Ok(Box::new(at::At {})),
        "BE" | "Be" | "be" => Ok(Box::new(be::Be {})),
        "DE" | "De" | "de" => Ok(Box::new(de::De {})),
        "NL" | "Nl" | "nl" => Ok(Box::new(nl::Nl {})),
        _ => Err(String::from("Failure: Country specified is not supported.")),
    }
}
