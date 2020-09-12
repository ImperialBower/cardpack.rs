pub trait Valuable {
    fn revise_value(&mut self, new_value: u8);

    fn get_value(&self) -> u8;
}