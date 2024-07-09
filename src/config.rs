use serde::Deserialize;
use std::{fs};

#[derive(Deserialize)]
pub struct Config{
    pub server:ServerConfig,
    pub client:ClientConfig,
}
#[derive(Deserialize)]
pub struct ServerConfig{
    pub ip:String,
    pub port:u16,
}
#[derive(Deserialize)]
pub struct ClientConfig{
    pub ip:String,
    pub port:u16,
}

pub fn read_config(file_path:&str)->Config{
    let context=fs::read_to_string(file_path).expect("Failed to read config file!");
    toml::from_str(&context).expect("Failed to read config file!")
}