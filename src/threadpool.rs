use std::{sync::{mpsc::{self}, Arc, Mutex}, thread};



pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>
}
type Job=Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {//TODO
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let (sender,receiver)=mpsc::channel();

        let receiver =Arc::new(Mutex::new(receiver));

        let mut workers=Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        // 使用 Vec<thread::JoinHandle<()>> 来存储线程，同时设定了容量上限 
        // with_capacity(size)，该方法还可以提前分配好内存空间，
        // 比 Vec::new 的性能要更好一点。

        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self,f:F)
    where 
        F:FnOnce() + Send + 'static,
    {
        let job=Box::new(f);
        self.sender.send(job).unwrap();
    }

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    // 闭包作为参数时可以由三个特征进行约束: Fn、FnMut 和 FnOnce
}

struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>,
}

impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread=thread::spawn(move|| loop {
            let job=receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}