use ::scraper::{Html, Selector};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/scraper")]
async fn scraper() -> impl Responder {
    let response = retrieve_html().await;
    let parsed_response = parse_html(&response);
    HttpResponse::Ok().body(parsed_response)
}
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(scraper)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
