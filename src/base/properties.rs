use std::{collections::HashMap, fs, sync::{Mutex, MutexGuard, OnceLock}};

static PROPERTIES_INSTANCE:OnceLock<Mutex<Properties>>=OnceLock::new();

#[derive(Debug)]
pub struct Properties{// 基础，除非完善自己，否则不变
    pub map:HashMap<String,String>
}

impl Properties {
    pub fn get_instance()->MutexGuard<'static,Properties>{
        PROPERTIES_INSTANCE.get().expect("Properties 还未初始化").lock().unwrap()
    }
    // 获取指定键的值，如果没有则返回默认值
    pub fn get(&self, key: &str, default_value: &str) -> String {
        self.map.get(key).unwrap_or(&default_value.to_string()).clone()
    }

    // 设置键值对
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // 打印所有键值对
    pub fn print(&self) {
        for (key, value) in &self.map {
            println!("{}={}", key, value);
        }
    }

    fn update_read_file(&mut self,file_path:&str){
        let contents=fs::read_to_string(file_path).expect("找不到配置文件!");//expect触发后，导致程序立即停止并退出
        for line in contents.lines() {
            let mut parts =line.splitn(2, '=');
            if let (Some(key),Some(value)) = (parts.next(),parts.next()) {
                self.set(key.trim().to_string(), value.trim().to_string());//trim() 用于去掉字符串两端的空白字符
            }
        }
    }

    // 初始化属性
    pub fn init(config_filepath:&str) {
        // 确保 `PROPERTIES_INSTANCE` 已初始化
        let mut properties = Properties {
            map: HashMap::new(),
        };

        // 更新属性
        println!("config_filepath:{:?}",config_filepath);
        properties.update_read_file(&config_filepath);

        // 打印所有属性
        properties.print();

        // 设置全局实例
        PROPERTIES_INSTANCE.set(Mutex::new(properties)).unwrap();
    }
}
// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        Properties::init("config/atomic.conf");
        let mut props = Properties::get_instance();
        props.set("key1".to_string(), "value1".to_string());
        props.set("key2".to_string(), "value2".to_string());
        assert_eq!(props.get("key1", "default_value"), "value1");
        assert_eq!(props.get("key2", "default_value"), "value2");
        assert_eq!(props.get("non_existent_key", "default_value"), "default_value");
        props.print();
        println!("test_properties ok!");
    }

    #[test]
    fn test_update_read_file() {
        Properties::init("config/atomic.conf");
        let mut props = Properties::get_instance();
        props.update_read_file("test/test_update_read_file.conf"); // 使用一个测试文件路径
        assert_eq!(props.get("key1", "default_value1"), "value1");
        assert_eq!(props.get("key2", "default_value2"), "value2");
        println!("test_update_read_file ok!");
    }
}