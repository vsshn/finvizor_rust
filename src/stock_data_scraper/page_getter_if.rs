pub trait PageGetterIf {
    fn get_next_page(&mut self) -> Option<String>;
}
