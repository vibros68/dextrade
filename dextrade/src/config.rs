use docopt::Docopt;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read};
use config_file::FromConfigFile;
use std::result::{Result};

use crate::dex_error;

const USAGE: &'static str = "
Dex Trade.

Usage:
  dextrade run <config-path> [options]
  dextrade (-h | --help)

Options:
  -h, --help    Show this message.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    cmd_run: bool,
    arg_config_path: String,
}

pub fn get_config() ->Result<Config, dex_error::DexError> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let config_file = match ConfigFile::from_config_file(args.arg_config_path) {
        Ok(cf) => cf,
        Err(e) => return Err(dex_error::new("read config file".to_string(),e.to_string())),
    };

    let mut buff = String::new();
    let mut file = File::open(config_file.secret_key_path).unwrap();
    file.read_to_string(&mut buff).unwrap();
    let datas: Vec<u8> = serde_json::from_str(&mut buff).unwrap();
    if datas.len() != 64 {
        return Err(dex_error::new("read secret key".to_string(),"invalid secret key format".to_string()))
    }
    let secret_key = datas.try_into()
        .unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 64, v.len()));
    let config = Config{ secret_key, swap: config_file.swap };
    Ok(config)
}

#[derive(Debug)]
pub struct Config {
    pub secret_key :[u8; 64],
    pub swap: Swap
}

#[derive(Deserialize)]
struct ConfigFile {
    secret_key_path: String,
    swap: Swap
}

#[derive(Debug,Deserialize)]
pub struct Swap {
    pub from_symbol: String,
    pub to_symbol: String,
    pub from_amount: f64,
}