use ::scraper::{Html, Selector};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
async fn retrieve_html() -> String {
    let response = reqwest::get("https://news.ycombinator.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return response;
}
async fn extract_titles() -> Vec<String> {
    let (doc_body, title) = select_el(".titleline").await;

    let mut titles = Vec::new(); // declare empty vector to hold titles
    for title in doc_body.select(&title) {
        let title_text = title.text().collect::<Vec<_>>(); // Push each title onto the Vec after converting it to a String
        if !title_text.is_empty() {
            titles.push(String::from(title_text[0]))
        }
    }
    titles
}

async fn select_el(selector: &str) -> (Html, Selector) {
    let response = retrieve_html().await;
    let document = Html::parse_document(&response);
    let title = Selector::parse(selector).unwrap();
    (document, title)
}

#[get("/")]
async fn scraper() -> impl Responder {
    let parsed_response = extract_titles().await;
    HttpResponse::Ok().json(parsed_response)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scraper))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
