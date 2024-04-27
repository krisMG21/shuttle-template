use ::scraper::{Html, Selector};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

async fn retrieve_html() -> String {
    let response = reqwest::get("https://www.scrapethissite.com/pages/simple")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return response;
}
async fn extract_country_names() -> Vec<String> {
    let response = retrieve_html().await;
    let country_name_selector = select_el(".country-name").await;
    let document = Html::parse_document(&response);
    let mut countries: Vec<String> = Vec::new(); // declare empty vector to hold names
    for country in document.select(&country_name_selector) {
        let country_name = country.text().collect::<String>().trim().to_owned(); // collect the text from the element and trim the whitespace
        countries.push(country_name);
    }
    countries.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase())); // sort the names alphabetically
    countries
}

async fn select_el(selector: &str) -> Selector {
    let country_name_selector = Selector::parse(selector).unwrap();
    country_name_selector
}

#[get("/")]
async fn scraper() -> impl Responder {
    let parsed_response = extract_country_names().await;
    HttpResponse::Ok().json(parsed_response)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scraper))
        .bind("127.0.0.1:8080")?
        .workers(4) // turn this into a multi-thread server
        .run()
        .await
}
