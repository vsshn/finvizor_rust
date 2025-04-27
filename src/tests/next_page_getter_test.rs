use crate::scraper::{next_page_getter, next_page_getter_if::NextPageGetterIf};

const BASE_PAGE: &str = "www.vshn.com/";

#[test]
fn test_next_page_getter_normal() {
    assert_eq!(
        next_page_getter::NextPageGetter.get_next_page(BASE_PAGE, 1),
        format!("{}&r=21", BASE_PAGE)
    );
    assert_eq!(
        next_page_getter::NextPageGetter.get_next_page(BASE_PAGE, 3),
        format!("{}&r=61", BASE_PAGE)
    );
}
