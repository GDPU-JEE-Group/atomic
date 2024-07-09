mod mutex;
mod condvar;
mod semaphore;
mod spinlock;
mod config;

pub use mutex::Mutex;
pub use spinlock::SpinLock;
pub use condvar::CondVar;
pub use semaphore::Semaphore;
pub use config::Config;
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
