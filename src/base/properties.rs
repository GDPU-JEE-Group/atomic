use std::{collections::HashMap, fs, sync::{Mutex, MutexGuard, OnceLock}};

static PROPERTIES_INSTANCE: OnceLock<Mutex<Properties>> = OnceLock::new();

#[derive(Debug)]
pub struct Properties {
    pub map: HashMap<String, String>,
}

impl Properties {
    fn get_instance() -> MutexGuard<'static, Properties> {
        PROPERTIES_INSTANCE.get().expect("Properties 还未初始化").lock().unwrap()
    }

    // 静态方法，获取指定键的值，如果没有则返回默认值
    pub fn get(key: &str, default_value: &str) -> String {
        let properties = Properties::get_instance();
        properties.map.get(key).unwrap_or(&default_value.to_string()).trim_matches('"').to_string()
    }

    // 静态方法，设置键值对
    pub fn set(key: String, value: String) {
        let mut properties = Properties::get_instance();
        properties.map.insert(key, value);
    }

    // 打印所有键值对
    pub fn print() {
        let properties = Properties::get_instance();
        println!("所有键值对：");
        for (key, value) in &properties.map {
            println!("  {}={}", key, value);
        }
    }

    // 从文件中更新属性
    fn update_read_file(file_path: &str) {
        let mut properties = Properties::get_instance();
        let contents = fs::read_to_string(file_path).expect("找不到配置文件!");
        for line in contents.lines() {
            let mut parts = line.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                properties.map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    // 初始化属性
    pub fn init(config_filepath: &str) {
        let properties = Properties {
            map: HashMap::new(),
        };
        // 设置全局实例
        PROPERTIES_INSTANCE.set(Mutex::new(properties)).unwrap();
        // 更新属性
        println!("config_filepath: {:?}", config_filepath);
        Properties::update_read_file(&config_filepath);
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        Properties::init("config/atomic.conf");
        Properties::set("key1".to_string(), "value1".to_string());
        Properties::set("key2".to_string(), "value2".to_string());
        assert_eq!(Properties::get("key1", "default_value"), "value1");
        assert_eq!(Properties::get("key2", "default_value"), "value2");
        assert_eq!(Properties::get("non_existent_key", "default_value"), "default_value");
        Properties::print();
        println!("test_properties ok!");
    }

    #[test]
    fn test_update_read_file() {
        Properties::init("config/atomic.conf");
        Properties::update_read_file("test/test_update_read_file.conf");
        assert_eq!(Properties::get("key1", "default_value1"), "value1");
        assert_eq!(Properties::get("key2", "default_value2"), "value2");
        println!("test_update_read_file ok!");
    }
}
