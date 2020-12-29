use std::{fs::File, io::prelude::*};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub border: BorderConfig,
}
#[derive(Deserialize, Debug)]
pub struct BorderConfig {
    pub active: [u16; 3],
    pub inactive: [u16; 3],
    pub width: u8,
}

impl Config {
    pub fn new() -> Self {
        let mut file = String::new();
        toml::from_str::<Config>({
            File::open("dwmir.toml")
                .expect("unable to open config")
                .read_to_string(&mut file)
                .expect("unable to read config.toml");
            &file[..]
        })
        .expect("unable to parse config.toml")
    }
}
