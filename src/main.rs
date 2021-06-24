//use calamine::Error;
//use serde::{Deserialize, Serialize};
use iban_beaver::country::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    // verifies iban
    let verify_iban = warp::any()
        .and(warp::path("iban"))
        .and(warp::path::param::<String>())
        .map(|iban| warp::reply::json(&verify_request(&iban)));

    // attempts to download new data and fill database
    let database_update = warp::any()
        .and(warp::path!("db" / "update" / String))
        .map(|country| warp::reply::json(&update_table_request(&country)));

    // fills database with data that is already downloaded
    let fill_database = warp::any()
        .and(warp::path!("db" / "fill" / String))
        .map(|country| warp::reply::json(&fill_table_request(&country)));

    // blacklists iban
    let blacklist = warp::any()
        .and(warp::path!("db" / "blacklist" / String / String))
        .map(|iban, op| warp::reply::json(&blacklist_request(&iban, &op)));

    let routes = warp::get().and(
        verify_iban
            .or(database_update)
            .or(fill_database)
            .or(blacklist),
    );
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
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
