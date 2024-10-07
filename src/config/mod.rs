use config_file::FromConfigFile;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::{collections::HashMap, error::Error, path::Path};

const CONFIG_PATH: &str = "conductor.toml";
type Config = HashMap<String, Printer>;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Printer {
    printer_type: String, //printerType,

    vars: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum PrinterType {
    Bambu(PrinterSubType),
    Klipper,
}

#[derive(Serialize, Deserialize, Debug)]
enum PrinterSubType {
    X1C,
    P1,
    A1,
}

pub(crate) fn deser() -> Result<Config, Box<dyn Error>> {
    match Config::from_config_file("config.toml") {
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
            println!("here");
            return Some(e.into());
        }
    };

    // Write some data to the file
    if let Err(e) = file.write_all(cfg.as_bytes()) {
        println!("here22");
        return Some(e.into());
    } else {
        return None;
    }
}
