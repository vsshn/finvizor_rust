pub mod ticker_scraper {
    pub mod next_page_getter;
    pub mod next_page_getter_if;
    pub mod scrape;
    pub mod ticker_parser;
    pub mod ticker_parser_if;
}

pub mod ticker_data_scraper {
    pub mod data_scrape;
    pub mod next_page_getter;
    pub mod ticker_data_parser;
    pub mod ticker_data_parser_if;
}

pub mod fetcher {
    pub mod fetcher;
    pub mod fetcher_if;
}

pub mod ticker_data {
    pub mod ticker_data;
}

#[cfg(test)]
pub mod tests {
    pub mod next_page_getter_test;
    pub mod scrape_test;
    pub mod ticker_parser_test;
}
