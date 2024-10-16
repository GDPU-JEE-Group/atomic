use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

type Job=Box<dyn FnOnce() + Send+ 'static>;

struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}
/*  
Worker：
    每个 Worker 通过 receiver 不断接收任务，接收到任务后会执行任务。
    如果通道关闭或出错，Worker 将退出循环，并结束线程
new（）：
    这是 Worker 的构造函数，负责创建一个新的工作线程。
receiver:Arc<Mutex<mpsc::Receiver<Job>>>：    
    通过多生产者-单消费者通道 (mpsc::Receiver) 接收任务队列中的任务，并通过互斥锁 (Mutex) 确保线程安全。Arc 是原子引用计数，用于共享通道接收器给多个线程。
recv()：
    从通道中接收任务，返回一个 Result，其中 Ok(job) 表示成功接收到任务，Err(_) 表示通道关闭，停止工作。
unwrap()：
    是为了在锁失败时触发 panic。//TODO
job():
    真正执行任务的地方

//?`Arc` 在内部维护了一个原子引用计数，每次克隆 `Arc` 时，这个计数会增加。当某个 `Arc` 实例被丢弃时，计数减少。只有当计数降为 0 时，数据才会被释放。这种机制可以防止在多线程环境中，多个线程尝试同时释放同一块内存时发生的问题。
*/
impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{//receiver是消费者的通道
        let thread =thread::spawn(move||loop {
           let message =receiver.lock().unwrap().recv();//?取任务时加锁，防止其他线程干扰，lock()时上锁，recv()结束时释放锁，保证仅有获取任务这个区间才锁。
            //？let 的作用域就是分号，while let 的作用域变成了while最后的 ,所以receiver.lock().unwrap().recv()；会在分号解锁
           match message {
            Ok(job) => {
                println!("Worker {id} got a job; executing.");

                job();//真正执行任务的地方
            }
            Err(_) => {
                println!("Worker {id} disconnected; shutting down.");
                break;
            }
           } 
        });

        Worker { id: id, thread: Some(thread) }
    }
}

/* 
workers: 
    Vec<Worker>：保存了多个 Worker，每个 Worker 代表一个工作线程。
sender: 
    Option<mpsc::Sender<Job>>：mpsc 通道的发送端，用来向线程池发送任务。使用 Option 是为了方便在 drop 时可以将其设置为 None 来关闭通道。 
*/
pub struct ThreadPool {
    workers: Vec<Worker>,           // 保存所有的工作线程
    sender: Option<mpsc::Sender<Job>>,  // 任务发送器，负责向工作线程传递任务
}

impl ThreadPool {
    pub fn new(size:usize)->ThreadPool{
        /*
        ThreadPool是1个发送者，多个work，1work1接受者（消费者）
        mpsc::channel()，获取1发送者1接收者（彼此联通），发送者用Option包住直接成为成员，接收者clone几份基于每个worker1个
        接收者因为都是1个克隆的，所以任务是共享的，加锁的，arc智能指针，只有计数为0时才会回收这个共享资源

        //? 共享任务如果保证不会冲突？因为1个worker1个线程1个接受者，把不同线程jia
         */
        assert!(size > 0, "ThreadPool size must be greater than 0, got {}", size);
        let (sender,receiver)=mpsc::channel();
        let receiver=Arc::new(Mutex::new(receiver));
        let mut workers =Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender:Some(sender)
        }
    }
    /*
    sender.as_ref().unwrap().send(job).unwrap():
        是空的或发送失败，触发 panic。//! TODO
     */
    pub fn execute<F>(&self,f:F)//泛型,execute 是一个泛型方法，可以接收任何符合 FnOnce() + Send + 'static 约束的函数或闭包。
    where 
        F:FnOnce()+Send+'static,//约束
    {
        let job=Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());//take() 方法会取出 Option 中的值,将 Option 变为 None

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();//join() 方法时，它会等待线程完成并进行资源清理。
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 引入上层模块的内容
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4); // 创建一个包含4个工作线程的线程池

        let result = Arc::new(Mutex::new(Vec::new())); // 用于收集结果的共享向量

        for i in 0..10 {
            let result = Arc::clone(&result); // 克隆 Arc，传递到线程中
            pool.execute(move || {
                thread::sleep(Duration::from_millis(100)); // 模拟工作
                result.lock().unwrap().push(i); // 将任务编号存入结果向量
            });
        }

        // 等待一段时间，让所有工作线程完成
        thread::sleep(Duration::from_millis(200));

        // 验证结果是否包含所有任务
        let mut results = result.lock().unwrap();
        results.sort(); // 排序以确保顺序
        assert_eq!(*results, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    #[should_panic(expected = "ThreadPool size must be greater than 0")]
    fn test_thread_pool_zero_size() {
        ThreadPool::new(0); // 期望在此处引发 panic
    }
}




