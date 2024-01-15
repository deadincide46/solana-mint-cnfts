use std::{path::Path, fs::{File, OpenOptions}, io::BufReader};

use crate::structs::Config;







pub fn read_config() -> Config {
    let config_path = Path::new("./src/files/config.json");
    let file = File::open(config_path).expect("cant open config");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("cant read config: Conifg");
    return config;
}



pub fn write_config(config: &Config) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./src/files/config.json")
        .expect("Unable to open config.json for writing");
    
    serde_json::to_writer_pretty(file, config).expect("Error writing config.json");
}