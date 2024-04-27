use std::collections::BTreeMap;

use ::scraper::{Html, Selector};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
#[derive(Serialize)]
struct Country {
    name: String,
    capital: String,
}
async fn retrieve_html() -> String {
    let response = reqwest::get("https://www.scrapethissite.com/pages/simple")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return response;
}
async fn extract_countries() -> BTreeMap<String, String> {
    let response = retrieve_html().await;
    let country_name_selector = select_el(".country-name").await;
    let capital_selector = select_el(".country-capital").await;
    let document = Html::parse_document(&response);
    let mut countries: Vec<String> = Vec::new(); // declare empty vector to hold names
    let mut capitals: Vec<String> = Vec::new();
    for country in document.select(&country_name_selector) {
        let country_name = country.text().collect::<String>().trim().to_owned(); // collect the text from the element and trim the whitespace
        countries.push(country_name);
    }
    for capital in document.select(&capital_selector) {
        let country_capital = capital.text().collect::<String>().trim().to_owned();
        capitals.push(country_capital);
    }
    let table = BTreeMap::from_iter(countries.into_iter().zip(capitals.into_iter()));

    table
}

async fn select_el(selector: &str) -> Selector {
    let country_name_selector = Selector::parse(selector).unwrap();
    country_name_selector
}

#[get("/")]
async fn scraper() -> impl Responder {
    let parsed_response = extract_countries().await;
    let mut countries: Vec<Country> = vec![];
    for (country, capital) in parsed_response.iter() {
        countries.push(Country {
            name: country.to_string(),
            capital: capital.to_string(),
        })
    }
    HttpResponse::Ok().json(countries)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scraper))
        .bind("127.0.0.1:8080")?
        .workers(4) // turn this into a multi-thread server
        .run()
        .await
}
