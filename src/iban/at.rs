// Data: https://www.oenb.at/docroot/downloads_observ/sepa-zv-vz_gesamt.csv
pub fn bank_code(iban: &String) -> i32 {
    iban.chars()
        .skip(4)
        .take(5)
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

// Countries have different valid lengths of iban
pub fn verify_length(iban: &String) -> bool {
    let nof_chars = iban.chars().count();
    nof_chars == 20
}
