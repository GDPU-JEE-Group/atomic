use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use chrono::Local;
use colored::*;
use crossbeam_channel::{bounded, Receiver, Sender};

// 日志级别
#[derive(Debug)]
pub enum LogLevel {
    Verbose,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LogLevel {
    fn to_str(&self) -> &str {
        match self {
            LogLevel::Verbose => "V",
            LogLevel::Info => "I",
            LogLevel::Warning => "W",
            LogLevel::Error => "E",
            LogLevel::Fatal => "F",
        }
    }

    fn to_color(&self, message: &str) -> ColoredString {
        match self {
            LogLevel::Warning => message.yellow(),
            LogLevel::Error | LogLevel::Fatal => message.red(),
            _ => message.normal(),
        }
    }
}

// 日志条目
struct LogEntry {
    timestamp: String,
    level: LogLevel,
    tag: String,
    message: String,
}

// 单例Log类
#[derive(Debug)]
pub struct Log {
    sender: Sender<LogEntry>,
    max_pool_size: usize,
    max_buffer_size: usize,
    log_file: Arc<Mutex<Option<File>>>,
}

static LOG_INSTANCE: OnceLock<Mutex<Log>> = OnceLock::new();

impl Log {
    // 初始化单例
    pub fn init(max_pool_size: usize, max_buffer_size: usize, log_dir: PathBuf) {
        let log = Log::new(max_pool_size, max_buffer_size, log_dir);
        LOG_INSTANCE.set(Mutex::new(log)).unwrap();
    }

    // 创建新的日志对象
    fn new(max_pool_size: usize, max_buffer_size: usize, log_dir: PathBuf) -> Self {
        let (sender, receiver) = bounded(max_pool_size);
        let log_file = Arc::new(Mutex::new(None));

        // 确保日志目录存在
        let current_date = Local::now().date_naive();
        Self::update_log_file(&log_dir, current_date, &log_file);

        // 启动异步线程处理日志
        let file_handle = Arc::clone(&log_file);
        thread::spawn(move || Self::log_thread(receiver, max_buffer_size, log_dir, file_handle));

        Log {
            sender,
            max_pool_size,
            max_buffer_size,
            log_file,
        }
    }

    // 单例获取
    fn get_instance() -> &'static Mutex<Log> {
        LOG_INSTANCE.get().expect("Log is not initialized")
    }

    // 日志输出
    pub fn log(level: LogLevel, tag: &str, message: &str) {
        let log = Log::get_instance().lock().unwrap();
        let timestamp = Local::now().format("%m-%d %H:%M:%S%.3f").to_string();
        let entry = LogEntry {
            timestamp,
            level,
            tag: tag.to_string(),
            message: message.to_string(),
        };

        if let Err(_) = log.sender.try_send(entry) {
            eprintln!("Log buffer is full, dropping log message.");
        }
    }

    // Debug级别日志
    pub fn d(tag: &str, message: &str) {
        Self::log(LogLevel::Verbose, tag, message);
    }

    // Info级别日志
    pub fn i(tag: &str, message: &str) {
        Self::log(LogLevel::Info, tag, message);
    }

    // Warning级别日志
    pub fn w(tag: &str, message: &str) {
        Self::log(LogLevel::Warning, tag, message);
    }

    // Error级别日志
    pub fn e(tag: &str, message: &str) {
        Self::log(LogLevel::Error, tag, message);
    }

    // Fatal级别日志
    pub fn f(tag: &str, message: &str) {
        Self::log(LogLevel::Fatal, tag, message);
    }

    // 日志处理线程
    fn log_thread(
        receiver: Receiver<LogEntry>,
        max_buffer_size: usize,
        log_dir: PathBuf,
        log_file: Arc<Mutex<Option<File>>>,
    ) {
        let mut last_date = Local::now().date_naive();
        let mut buffer_pool = Vec::new();

        loop {
            if let Ok(entry) = receiver.recv_timeout(Duration::from_secs(1)) {
                let current_date = Local::now().date_naive();
                if current_date != last_date {
                    Self::update_log_file(&log_dir, current_date, &log_file);
                    last_date = current_date;
                }

                println!(
                    "{} {} {} {}",
                    entry.timestamp.green(),
                    entry.level.to_color(&entry.level.to_str()),
                    entry.tag.purple(),
                    entry.message
                );

                if let Some(ref mut file) = *log_file.lock().unwrap() {
                    let log_line = format!(
                        "{} {} {} {}\n",
                        entry.timestamp, entry.level.to_str(), entry.tag, entry.message
                    );
                    if log_line.len() <= max_buffer_size {
                        if let Err(e) = file.write_all(log_line.as_bytes()) {
                            eprintln!("Failed to write to log file: {}", e);
                        }
                    }
                }

                buffer_pool.push(entry);
                if buffer_pool.len() > max_buffer_size {
                    buffer_pool.remove(0);
                }
            }
        }
    }

    // 更新日志文件
    fn update_log_file(log_dir: &PathBuf, date: chrono::NaiveDate, log_file: &Arc<Mutex<Option<File>>>) {
        let filename = format!("{}/log_{}.log", log_dir.to_string_lossy(), date.format("%Y%m%d"));
        let new_file = OpenOptions::new().append(true).create(true).open(&filename);

        match new_file {
            Ok(file) => {
                let mut log_file_guard = log_file.lock().unwrap();
                *log_file_guard = Some(file);
            }
            Err(e) => {
                eprintln!("Failed to create log file: {}", e);
            }
        }
    }
}
