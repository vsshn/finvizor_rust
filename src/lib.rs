pub mod ticker_scraper {
    pub mod next_page_getter;
    pub mod next_page_getter_if;
    pub mod scrape;
    pub mod ticker_parser;
    pub mod ticker_parser_if;
}

pub mod fetcher {
    pub mod fetcher;
    pub mod fetcher_if;
}

#[cfg(test)]
pub mod tests {
    pub mod next_page_getter_test;
    pub mod scrape_test;
    pub mod ticker_parser_test;
}
