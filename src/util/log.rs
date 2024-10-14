use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
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
    message: String,
}

// 日志类
pub struct Log {
    sender: Sender<LogEntry>,
    max_pool_size: usize,
    max_buffer_size: usize,
    log_file: Arc<Mutex<Option<File>>>,
}

impl Log {
    pub fn new(max_pool_size: usize, max_buffer_size: usize, log_dir: PathBuf) -> Self {
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

    pub fn log(&self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%m-%d %H:%M:%S%.3f").to_string();
        let entry = LogEntry {
            timestamp,
            level,
            message: message.to_string(),
        };

        if let Err(_) = self.sender.try_send(entry) {
            // 缓冲池已满，丢弃日志
            eprintln!("Log buffer is full, dropping log message.");
        }
    }

    fn log_thread(
        receiver: Receiver<LogEntry>,
        max_buffer_size: usize,
        log_dir: PathBuf,
        log_file: Arc<Mutex<Option<File>>>,
    ) {
        let mut last_date = Local::now().date_naive();
        let mut buffer_pool = Vec::new();

        loop {
            // 接收日志条目
            if let Ok(entry) = receiver.recv_timeout(Duration::from_secs(1)) {
                let current_date = Local::now().date_naive();
                if current_date != last_date {
                    // 日期变化，更新日志文件
                    Self::update_log_file(&log_dir, current_date, &log_file);
                    last_date = current_date;
                }

                // 控制台输出
                println!(
                    "{} {}",
                    entry.timestamp,
                    entry.level.to_color(&format!("{} {}", entry.level.to_str(), entry.message))
                );

                // 写入文件
                if let Some(ref mut file) = *log_file.lock().unwrap() {
                    let log_line = format!("{} {} {}\n", entry.timestamp, entry.level.to_str(), entry.message);
                    if log_line.len() <= max_buffer_size {
                        if let Err(e) = file.write_all(log_line.as_bytes()) {
                            eprintln!("Failed to write to log file: {}", e);
                        } else {
                            // 强制写入磁盘
                            file.sync_all().expect("Failed to sync log file");
                        }
                    }
                }

                // 控制缓冲池大小
                buffer_pool.push(entry);
                if buffer_pool.len() > max_buffer_size {
                    buffer_pool.remove(0); // 丢弃最早的日志条目
                }
            }
        }
    }

    fn update_log_file(log_dir: &PathBuf, date: chrono::NaiveDate, log_file: &Arc<Mutex<Option<File>>>) {
        let filename = format!("{}/log_{}.log", log_dir.to_string_lossy(), date.format("%Y%m%d"));
        let new_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&filename);

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