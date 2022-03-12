use crate::country::get_country;
use crate::country::BankData;
use crate::db::{establish_connection, is_blacklisted};
use iban::*;
//use core::convert::TryFrom;
pub fn parse(iban_str: &str) -> Result<Iban, String> {
    iban_str.parse::<Iban>().map_err(|e| e.to_string())
}
pub fn verify(iban: Iban) -> Result<BankData, String> {
    let country = get_country(iban.country_code()).map_err(|e| e.to_string())?;
    let connection = &establish_connection();
    is_blacklisted(connection, &iban.to_string())?;
    if let Some(bank_code) = iban.bank_identifier() {
        let bank_data = country.get_bank_data(connection, &bank_code)?;
        Ok(bank_data)
    } else {
        Err(String::from("No bank code found in IBAN"))
    }
}
#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    fn parse_iban_checksum() {
        let de_ok = "DE27100777770209299700".parse::<Iban>();
        let de_err_math = "DE27100777770209299704".parse::<Iban>();
        let de_err_length = "DE2710077777209299700".parse::<Iban>();
        assert!(de_ok.is_ok());
        assert!(de_err_math.is_err());
        assert!(de_err_length.is_err());
    }

    #[test]
    fn parse_iban_country_code() {
        let de_ok = "DE27100777770209299700".parse::<Iban>();
        let de_err = "XE27100777770209299700".parse::<Iban>();
        assert!(de_ok.is_ok());
        assert!(de_err.is_err());
    }

    #[test]
    #[serial]
    fn verify_iban() {
        let iban = "DE27100777770209299700".parse::<Iban>().unwrap();
        let de_ok = verify(iban).unwrap();
        assert!(de_ok.code.eq("10077777"));
        assert!(de_ok.bic.eq(&Some("NORSDE51XXX".to_string())));

        let iban = "BE68539007547034".parse::<Iban>().unwrap();
        let be_ok = verify(iban).unwrap();
        assert!(be_ok.code.eq("539"));
        assert!(be_ok.bic.eq(&Some("NAP".to_string())));
    }
}
