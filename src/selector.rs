use scraper::Selector;
pub async fn retrieve_html() -> String {
    let response = reqwest::get("https://www.scrapethissite.com/pages/simple")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return response;
}
pub async fn select_el(selector: &str) -> Selector {
    let selector = Selector::parse(selector).unwrap();
    selector
}
