// THIS FILE ISNT USED AT THE MOMENT
// Keeping it around for reference for now
use super::schema::*;

#[derive(Insertable, Queryable)]
#[table_name = "t_de"]
pub struct DeBankData {
    //#[serde(rename = "Datensatz-nummer")]
    id: i32,
    //#[serde(rename = "Bankleitzahl")]
    code: i32,
    //#[serde(rename = "Kurzbezeichnung")]
    name: String,
    //#[serde(rename = "PLZ")]
    zip: i32,
    //#[serde(rename = "Ort")]
    city: String,
    //#[serde(rename = "BIC")]
    bic: Option<String>,
}
impl From<super::de::BankData> for DeBankData {
    fn from(bd: super::de::BankData) -> Self {
        DeBankData {
            id: bd.id,
            code: bd.code,
            name: bd.name.clone(),
            zip: bd.zip,
            city: bd.city.clone(),
            bic: bd.bic.clone(), 
        }
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "blacklist"]
pub struct Blacklist {
    iban: String,
    blacklisted: bool,
}
