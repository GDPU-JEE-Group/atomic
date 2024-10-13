use std::{collections::HashMap, fs, sync::{Mutex, MutexGuard, OnceLock}};

static PROPERTIES_INSTANCE:OnceLock<Mutex<Properties>>=OnceLock::new();

#[derive(Debug)]
pub struct Properties{
    pub map:HashMap<String,String>
}

impl Properties {
    pub fn instance()->MutexGuard<'static,Properties>{
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

    pub fn update_read_file(&mut self,file_path:&str){
        let contents=fs::read_to_string(file_path).expect("找不到配置文件!");//expect触发后，导致程序立即停止并退出
        for line in contents.lines() {
            let mut parts =line.splitn(2, '=');
            if let (Some(key),Some(value)) = (parts.next(),parts.next()) {
                self.set(key.trim().to_string(), value.trim().to_string());//trim() 用于去掉字符串两端的空白字符
            }
        }
    }

    // 初始化属性
    pub fn init(args: Vec<String>) {
        // 确保 `PROPERTIES_INSTANCE` 已初始化
        let mut properties = Properties {
            map: HashMap::new(),
        };

        // 校验参数
        if args.len() != 3 || args[1] != "-c" {
            eprintln!("Usage: {} -c <config_file_path>", args[0]);
            std::process::exit(1);
        }

        // 获取配置文件路径
        let config_filepath = args[2].clone();

        // 更新属性
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
        let args = vec!["-c".to_string(), "config/atomic.conf".to_string()];

        Properties::init(args);
        let mut props = Properties::instance();
        props.set("key1".to_string(), "value1".to_string());
        props.set("key2".to_string(), "value2".to_string());
        assert_eq!(props.get("key1", "default_value"), "value1");
        assert_eq!(props.get("key2", "default_value"), "value2");
        assert_eq!(props.get("non_existent_key", "default_value"), "default_value");
        println!("test_properties ok!");
    }

    #[test]
    fn test_update_read_file() {
        let args = vec!["-c".to_string(), "config/atomic.conf".to_string()];

        Properties::init(args);
        let mut props = Properties::instance();
        props.update_read_file("test_properties.txt"); // 使用一个测试文件路径
        assert_eq!(props.get("key1", "default_value"), "value1");
        assert_eq!(props.get("key2", "default_value"), "value2");
        println!("test_update_read_file ok!");
    }
}