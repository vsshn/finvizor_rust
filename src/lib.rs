pub mod ticker_scraper {
    pub mod scrape;
    pub mod ticker_parser;
    pub mod ticker_parser_if;
}

pub mod next_page_getter {
    pub mod next_page_getter;
    pub mod next_page_getter_if;
}

pub mod string_manipulation {
    pub mod string_manipulation;
}

pub mod stock_related_types {
    pub mod floating_point;
    pub mod security;
    pub mod ticker_data;
}

pub mod stock_data_scraper {
    pub mod data_parser;
    pub mod data_parser_if;

    pub mod page_getter;
    pub mod scrape_ticker_data;
}

pub mod filtering {
    pub mod dividend_filter;
    pub mod filter_if;
    pub mod pb_filter;
}

pub mod fetcher {
    pub mod fetcher;
    pub mod fetcher_if;
}

pub mod ticker_file_reader {
    pub mod ticker_file_reader;
}

#[cfg(test)]
pub mod tests {
    pub mod compound_filter_test;
    pub mod data_parser_test;
    pub mod dividend_filter_test;
    pub mod floating_point_test;
    pub mod next_page_getter_test;
    pub mod pb_filter_test;
    pub mod scrape_test;
    pub mod scrape_ticker_data_test;
    pub mod ticker_parser_test;
}
