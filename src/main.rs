use std::env;

use atomic::properties::Properties;

fn main() {
        // 初始化配置
    Properties::init(env::args().collect());
    let _context=Properties::instance();
    println!("Hello, world!");
}



