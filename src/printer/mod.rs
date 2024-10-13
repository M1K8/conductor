use std::collections::HashMap;

pub mod bambu;
pub mod moonraker;

pub struct PrintFile {}

pub trait Printer {
    fn print(f: &PrintFile) -> Result<(), Box<dyn std::error::Error>>;
    fn upload(f: &PrintFile) -> Result<(), Box<dyn std::error::Error>>;

    fn get_info() -> HashMap<String, String>;
}
