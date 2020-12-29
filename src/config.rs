use std::{error::Error, fs::File, io::prelude::*, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub border: BorderConfig,
}
#[derive(Deserialize)]
pub struct BorderConfig {
    pub active: Option<(u16, u16, u16)>,
    pub inactive: Option<(u16, u16, u16)>,
    pub width: u8,
}

impl Config {
    pub fn new() -> Self {
        let mut file = String::new();
        toml::from_str::<Config>({
            File::open(
                Path::new(&std::env::var("HOME").expect("no HOME found!"))
                    .join(".config/dwmir/config.toml"),
            )
            .expect("unable to open config")
            .read_to_string(&mut file)
            .expect("unable to read config.toml");
            &file[..]
        })
        .expect("unable to parse config.toml")
    }
}
