use crate::stock_data_scraper::{page_getter::PageGetter, page_getter_if::PageGetterIf};

#[test]
fn test_page_getter_normal() {
    let tickers = vec!["META".to_string(), "AAPL".to_string()];
    let base_page = "https://finviz.com/quote.ashx?t=".to_string();
    let mut page_getter = PageGetter::new(tickers, base_page);

    assert_eq!(
        page_getter.get_next_page(),
        Some("https://finviz.com/quote.ashx?t=META".to_string())
    );

    assert_eq!(
        page_getter.get_next_page(),
        Some("https://finviz.com/quote.ashx?t=AAPL".to_string())
    );

    assert_eq!(page_getter.get_next_page(), None);
    assert_eq!(page_getter.get_next_page(), None);
}

#[test]
fn test_page_getter_empty() {
    let mut page_getter = PageGetter::new(vec![], "".to_string());
    assert_eq!(page_getter.get_next_page(), None);
}
