use std::collections::HashMap;

pub mod bambu;
pub mod moonraker;

pub struct PrintFile {}

pub trait Printer {
    fn print(&self, f: &PrintFile) -> Result<(), Box<dyn std::error::Error>>;
    fn upload(&self, f: &PrintFile) -> Result<(), Box<dyn std::error::Error>>;

    fn get_info(&self) -> HashMap<String, String>;
}
