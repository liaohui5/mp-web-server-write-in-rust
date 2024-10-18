use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(unused)]
pub enum TaskMessage {
    NewTask(Job),
    Exit,
}

#[allow(unused)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<TaskMessage>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    TaskMessage::NewTask(job) => {
                        println!("worker-{}-execute-task.", id);
                        job();
                    }
                    TaskMessage::Exit => {
                        // 终止线程: 停止接收信道对象发送的任务消息
                        // 退出 loop 循环, 那么这个线程自然就执行完了
                        println!("worker-{}-exit.", id);
                        break;
                    }
                }
            }
        });

        let thread = Some(thread);

        Worker { id, thread }
    }
}

#[allow(unused)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<TaskMessage>,
}
impl ThreadPool {
    pub fn new(thread_count: usize) -> ThreadPool {
        assert!(thread_count > 0);
        let mut workers = Vec::with_capacity(thread_count);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(TaskMessage::NewTask(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
