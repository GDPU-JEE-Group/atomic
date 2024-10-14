use std::env;

use atomic::base::properties::Properties;
use atomic::util::log::Log;
use atomic::util::log::LogLevel;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    // Parameter processing


    // 属性Properties初始化
    Properties::init(env::args().collect());
    let _context=Properties::get_instance();
    _context.print();

    println!("日志Log初始化");
    // 日志Log初始化
    let log_dir = PathBuf::from("log");
        let logger = Log::new(100, 8000, log_dir);
    
        logger.log(LogLevel::Verbose, "sending type:0xfff40000 to /data/misc/sensor/sensor_ctrl");
        logger.log(LogLevel::Error, "set_timerslack_ns write failed: Operation not permitted");
        logger.log(LogLevel::Info, "chatty: uid=10096(com.tencent.tmgp.l10) UnityMain identical 17 lines");
    
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(6));

}



