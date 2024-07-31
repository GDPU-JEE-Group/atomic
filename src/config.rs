use serde::Deserialize;
use std::{fs, sync::Mutex};
use std::sync::{MutexGuard, OnceLock};
use std::ops::Deref;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub config_filepath: String,
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct ClientConfig {
    pub ip: String,
    pub port: u16,
}

// 使用 OnceLock 替代 Once + Mutex 组合，以确保线程安全
static CONFIG_INSTANCE: OnceLock<Mutex<Config>> = OnceLock::new();

// 获取全局单例实例的引用
pub fn instance() -> MutexGuard<'static, Config> {
    CONFIG_INSTANCE.get().expect("Config not initialized").lock().unwrap()
}

// 初始化配置
pub fn init_config(args: Vec<String>) {
    // 校验参数
    if args.len() != 3 || args[1] != "-c" {
        eprintln!("Usage: {} -c <config_file_path>", args[0]);
        std::process::exit(1);
    }

    // 读取并写入参数
    let config_filepath = args[2].clone();
    let mut config = read_config(&config_filepath);
    config.config_filepath = config_filepath;

    // 初始化全局配置实例
    CONFIG_INSTANCE.set(Mutex::new(config)).unwrap();

    // 
    print_config();
}

// 更新配置
pub fn update_config() {
    let mut instance = instance();
    let config = read_config(&instance.config_filepath);
    *instance = config;
}

// 打印配置
pub fn print_config() {
    let config_instance = instance();
    println!("Config: {:?}", config_instance.deref());
}

// 读取配置文件
pub fn read_config(file_path: &str) -> Config {
    let context=fs::read_to_string(file_path).expect("Failed to read config file!");
    toml::from_str(&context).expect("Failed to read config file!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        let config = read_config("test_config.toml");
        assert_eq!(config.server.ip, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.client.ip, "127.0.0.1");
        assert_eq!(config.client.port, 8081);
    }
}
