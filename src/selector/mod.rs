use scraper::Selector;

pub async fn select_el(selector: &str) -> Selector {
    let selector = Selector::parse(selector).unwrap();
    selector
}
