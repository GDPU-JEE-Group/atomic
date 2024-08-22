use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self};



pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:Option<mpsc::Sender<Job>>
}
type Job=Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {//TODO
    // 从一个通道channel中，创建一个 sender和一个receiver
    // 把receiver复制给每个worker
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let (sender_,receiver)=mpsc::channel();

        let receiver =Arc::new(Mutex::new(receiver));

        let mut workers=Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        // 使用 Vec<thread::JoinHandle<()>> 来存储线程，同时设定了容量上限 
        // with_capacity(size)，该方法还可以提前分配好内存空间，
        // 比 Vec::new 的性能要更好一点。

        ThreadPool{
            workers,
            sender:Some(sender_)
        }
    }

    // 创建一个任务job，sender发送这个任务job给worker
    pub fn execute<F>(&self,f:F)
    where 
        F:FnOnce() + Send + 'static,
    {
        let job=Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    // 闭包作为参数时可以由三个特征进行约束: Fn、FnMut 和 FnOnce
}
impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());//释放sender
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread)=worker.thread.take(){// tacke()可以把Option的Some移动走，只剩下None
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id:usize,
    thread: Option<thread::JoinHandle<()>> ,
}

// new的时候，设置好id，运行带锁的任务。
impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread_=thread::spawn(move|| loop {
            // let job=receiver.lock().unwrap().recv().unwrap();
            // println!("Worker {id} got a job; executing.");
            // job();
            let message=receiver.lock().unwrap().recv();
            match message {
                Ok(job)=>{
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_)=>{
                    println!("Worker {id} disconnected; shutting down.");
                    break;//exit loop
                }
            }
        });
        //即使thead.join了，loop还是会无限循环，要借用 channel 的 drop 机制：释放 sender发送端后，receiver 接收端会收到报错，然后再退出
        Worker { 
            id,
            thread:Some(thread_) 
        }
    }
}