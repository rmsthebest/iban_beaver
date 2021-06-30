use crate::country::get_country;
use crate::country::BankData;
use crate::db::{establish_connection, is_blacklisted};

pub fn verify(iban: &str) -> Result<BankData, String> {
    let country = get_country(iban)?;
    country.verify_length(iban)?;
    country.verify_mod(iban)?;
    let bank_code = country.bank_code(iban);
    let connection = &establish_connection();
    is_blacklisted(connection, iban)?;
    let bank_data = country.get_bank_data(connection, &bank_code)?;

    Ok(bank_data)
}

fn convert_to_int(reordered_iban: String) -> Result<u128, std::num::ParseIntError> {
    let thing = reordered_iban
        .chars()
        .map(char_to_num)
        .collect::<String>();
    thing.parse()
}
fn char_to_num(c: char) -> String {
    match c {
        'a' | 'A' => "10".to_string(),
        'b' | 'B' => "11".to_string(),
        'c' | 'C' => "12".to_string(),
        'd' | 'D' => "13".to_string(),
        'e' | 'E' => "14".to_string(),
        'f' | 'F' => "15".to_string(),
        'g' | 'G' => "16".to_string(),
        'h' | 'H' => "17".to_string(),
        'i' | 'I' => "18".to_string(),
        'j' | 'J' => "19".to_string(),
        'k' | 'K' => "20".to_string(),
        'l' | 'L' => "21".to_string(),
        'm' | 'M' => "22".to_string(),
        'n' | 'N' => "23".to_string(),
        'o' | 'O' => "24".to_string(),
        'p' | 'P' => "25".to_string(),
        'q' | 'Q' => "26".to_string(),
        'r' | 'R' => "27".to_string(),
        's' | 'S' => "28".to_string(),
        't' | 'T' => "29".to_string(),
        'u' | 'U' => "30".to_string(),
        'v' | 'V' => "31".to_string(),
        'w' | 'W' => "32".to_string(),
        'x' | 'X' => "33".to_string(),
        'y' | 'Y' => "34".to_string(),
        'z' | 'Z' => "35".to_string(),
        _ => c.to_string(),
    }
}

pub trait Iban {
    // Each country needs to implement these
    fn bank_code(&self, iban: &str) -> String;
    fn verify_length(&self, iban: &str) -> Result<(), String>;

    // These all have generics that work for at least some countires (replace if needed)
    fn verify_mod(&self, iban: &str) -> Result<(), String> {
        // first four to the end
        let reorderd_iban = format!(
            "{}{}",
            iban.chars().skip(4).collect::<String>(),
            iban.chars().take(4).collect::<String>()
        );
        if let Ok(integer_iban) = convert_to_int(reorderd_iban) {
            if integer_iban.rem_euclid(97) == 1 {
                Ok(())
            } else {
                Err("Failure: IBAN modulo is not 1".to_string())
            }
        } else {
            Err("Failure: IBAN has invalid characters inside".to_string())
        }
    }
}
