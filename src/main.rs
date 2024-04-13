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
fn parse_html(response: &String) -> String {
    let doc_body = Html::parse_document(&response);
    let title = Selector::parse(".titleline").unwrap();
    let mut result = String::new();
    for title in doc_body.select(&title) {
        let titles = title.text().collect::<Vec<_>>();
        if !titles.is_empty() {
            result.push_str(&titles[0]);
            result.push('\n');
        }
    }
    result
}

#[get("/scraper")]
async fn scraper() -> impl Responder {
    let response = retrieve_html().await;
    let parsed_response = parse_html(&response);
    HttpResponse::Ok().body(parsed_response)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scraper))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
