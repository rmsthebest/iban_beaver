#[macro_use]
extern crate rocket;
use iban_beaver::interface::*;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

// Verify and get the BIC for IBAN you enter
#[openapi]
#[get("/verify/<iban_str>")]
fn verify(iban_str: &str) -> Json<IbanResponse> {
    Json(verify_request(iban_str))
}

#[openapi]
#[get("/blacklist/<iban_str>/<add_or_remove>")]
fn blacklist(iban_str: &str, add_or_remove: &str) -> Json<DbResponse> {
    Json(blacklist_request(iban_str, add_or_remove))
}

#[openapi]
#[get("/update/<country_code>")]
fn update(country_code: &str) -> Json<DbResponse> {
    Json(update_table_request(country_code))
}

#[openapi]
#[get("/re-fill/<country_code>")]
fn fill(country_code: &str) -> Json<DbResponse> {
    Json(fill_table_request(country_code))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![verify, blacklist, update, fill])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

/*
NL: https://www.betaalvereniging.nl/wp-content/uploads/BIC-lijst-NL.xlsx
BE: https://www.nbb.be/de/zahlungssysteme/zahlungsnormen/bankidentifikationscodes
DE: https://www.bundesbank.de/de/aufgaben/unbarer-zahlungsverkehr/serviceangebot/bankleitzahlen/download---bankleitzahlen-602592
LU: https://www.abbl.lu/topic/iban-and-bic-codes/
CH: https://www.six-interbank-clearing.com/dam/downloads/bc-bank-master/bcbankenstamm_d.xls
LI:
AT: https://www.oenb.at/idakilz/kiverzeichnis?action=downloadAllData
*/
