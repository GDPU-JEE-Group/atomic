use std::env;

use atomic::base::properties::Properties;
use atomic::util::log::Log;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    // Parameter processing
    let env_args:Vec<String>=env::args().collect();
    if env_args.len() != 3 || env_args[1] != "-c" {
        eprintln!("Usage: {} -c <config_file_path>", env_args[0]);
        std::process::exit(1);
    }

    Properties::init(env_args[2].as_str());

    // 初始化日志系统，设置最大日志池大小和最大缓冲区大小
    Log::init(100, 1024, PathBuf::from("/snow/rust/atomic/log"));

    // 测试不同级别的日志
    Log::d("MainModule", "This is a debug message.");
    Log::i("MainModule", "This is an info message.");
    Log::w("MainModule", "This is a warning message.");
    Log::e("MainModule", "This is an error message.");
    Log::f("MainModule", "This is a fatal error message.");
    Properties::get_instance().print();
    
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(2));
}



