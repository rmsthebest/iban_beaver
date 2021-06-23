//use calamine::Error;
//use serde::{Deserialize, Serialize};
use iban_beaver::{db, iban};
use warp::Filter;

#[tokio::main]
async fn main() {
    // verifies iban
    let verify_iban = warp::any()
        .and(warp::path("iban"))
        .and(warp::path::param::<String>())
        .map(|iban| warp::reply::json(&iban::verify(&iban)));

    // attempts to download new data and fill database
    let database_update = warp::any()
        .and(warp::path!("db" / "update" / String))
        .map(|country| warp::reply::json(&db::update_database(country)));

    // fills database with data that is already downloaded
    let fill_database = warp::any()
        .and(warp::path!("db" / "fill" / String))
        .map(|country| warp::reply::json(&db::fill_database(country)));

    // blacklists iban
    let blacklist = warp::any()
        .and(warp::path!("db" / "blacklist" / String / String))
        .map(|iban, op| warp::reply::json(&iban::blacklist_request(iban, op)));

    let routes = warp::get().and(verify_iban.or(database_update).or(fill_database).or(blacklist));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
