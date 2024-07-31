mod mutex;
mod condvar;
mod semaphore;
mod spinlock;
mod config;
mod threadpool;


pub use mutex::Mutex;
pub use spinlock::SpinLock;
pub use condvar::CondVar;
pub use semaphore::Semaphore;
pub use config::Config;
pub use config::instance;
pub use config::init_config;
pub use config::update_config;
pub use config::print_config;
pub use config::read_config;
pub use threadpool::ThreadPool;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

