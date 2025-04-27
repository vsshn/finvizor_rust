pub trait NextPageGetterIf {
    fn get_next_page(&self, base_page: &str, page_num: u32) -> String;
}
