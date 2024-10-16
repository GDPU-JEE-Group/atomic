`Worker` 是一个线程池中工作线程的结构体，负责从共享的任务队列中获取并执行任务。以下是对代码的详细讲解：

### 1. `Worker` 结构体
```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```
- `id: usize`：表示工作线程的标识符，用于区分不同的 `Worker`。
- `thread: Option<thread::JoinHandle<()>>`：保存线程句柄 (`JoinHandle`)，允许在执行完任务后与主线程汇合，`Option` 类型是为了方便在某些情况下（例如线程关闭时）存储 `None`。

### 2. `Worker::new` 方法
```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
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
```
- `fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker`：
  - 这是 `Worker` 的构造函数，负责创建一个新的工作线程。
  - 参数：
    - `id: usize`：工作线程的 ID。
    - `receiver: Arc<Mutex<mpsc::Receiver<Job>>>`：通过多生产者-单消费者通道 (`mpsc::Receiver`) 接收任务队列中的任务，并通过互斥锁 (`Mutex`) 确保线程安全。`Arc` 是原子引用计数，用于共享通道接收器给多个线程。
  
- 线程部分：
  - `thread::spawn(move || loop { ... })`：为每个 `Worker` 线程启动一个无限循环的线程，该线程会持续等待从 `receiver` 中获取任务。
  - `receiver.lock().unwrap().recv()`：
    - `lock().unwrap()`：获取 `Mutex` 锁来访问任务队列，`unwrap()` 是为了在锁失败时触发 panic。
    - `recv()`：从通道中接收任务，返回一个 `Result`，其中 `Ok(job)` 表示成功接收到任务，`Err(_)` 表示通道关闭，停止工作。
  
- `match message`：
  - 如果接收到任务 (`Ok(job)`)，则打印任务信息并执行该任务 (`job()`)，任务类型是 `Box<dyn FnOnce() + Send + 'static>`，即封装了一个闭包任务，确保它可以在线程中执行。
  - 如果接收器断开连接 (`Err(_)`)，则打印消息并退出循环，意味着这个 `Worker` 线程停止工作。

### 3. 工作流程
- 每个 `Worker` 通过 `receiver` 不断接收任务，接收到任务后会执行任务。如果通道关闭或出错，`Worker` 将退出循环，并结束线程。
- 这个模型确保每个工作线程可以独立工作，并且能根据任务的接收情况动态调整其工作状态。

### 总结
`Worker` 类是线程池中执行任务的核心组件，每个 `Worker` 在单独的线程中运行，并从共享任务队列（通过 `receiver`）中接收任务并执行。