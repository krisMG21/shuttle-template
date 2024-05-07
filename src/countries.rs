use std::collections::BTreeMap;

use scraper::Html;
use serde::Serialize;

use super::selector::{retrieve_html, select_el};

pub async fn extract() -> BTreeMap<String, String> {
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
#[derive(Serialize)]
pub struct Country {
    pub name: String,
    pub capital: String,
}
