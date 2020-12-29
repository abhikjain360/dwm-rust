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

// impl Config {
//     pub fn new() -> Result<Self, Box<dyn Error>> {
//         toml::from_str::<Config>({
//             let mut file = String::new();
//             File::open(Path::new(&std::env::var("HOME")?).join(".config/dwmir/config.toml"))?
//                 .read_to_string(&mut file);
//             &file[..]
//         }).
//     }
// }
