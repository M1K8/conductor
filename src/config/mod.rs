use config_file::FromConfigFile;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::{collections::HashMap, error::Error};

use crate::printer;

const CONFIG_PATH: &str = "~/conductor.toml";
pub(crate) type Config = HashMap<String, Printer>;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Printer {
    printer_type: String,
    ip: String,
    vars: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum PrinterType {
    Bambu(PrinterSubType),
    Klipper,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum PrinterSubType {
    X1C,
    P1,
    A1,
}

pub(crate) fn deser() -> Result<Config, Box<dyn Error>> {
    match Config::from_config_file(CONFIG_PATH) {
        Ok(c) => Ok(c),
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn ser(cfg: Config) -> Option<Box<dyn Error>> {
    let cfg = match toml::to_string(&cfg) {
        Ok(c) => c,
        Err(e) => return Some(e.into()),
    };

    let mut home = match home::home_dir() {
        Some(path) => path,
        None => {
            return None;
        }
    };

    home.push(CONFIG_PATH);

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&home)
    {
        Ok(file) => file,
        Err(e) => {
            return Some(e.into());
        }
    };

    // Write some data to the file
    if let Err(e) = file.write_all(cfg.as_bytes()) {
        return Some(e.into());
    } else {
        return None;
    }
}

fn get_printer_cfg<'a>(cfg: &'a Config, name: &'a str) -> Option<&'a Printer> {
    cfg.get(name)
}

pub(crate) fn get_printer(cfg: &Config, name: &str) -> Option<Box<dyn printer::Device>> {
    None
}

pub(crate) fn get_printer_type(s: String) -> Option<PrinterType> {
    match s.to_ascii_lowercase().as_str() {
        "x1c" => Some(PrinterType::Bambu(PrinterSubType::X1C)),
        "p1" => Some(PrinterType::Bambu(PrinterSubType::P1)),
        "a1" => Some(PrinterType::Bambu(PrinterSubType::A1)),
        "klipper" => Some(PrinterType::Klipper),
        _ => None,
    }
}
