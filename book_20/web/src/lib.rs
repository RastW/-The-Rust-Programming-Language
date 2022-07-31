use std::{thread, sync::{mpsc, Mutex}};
use std::sync::Arc;

pub struct ThreadPool {
    worker: Vec<Worker>,
    Sender: mpsc::Sender<Job>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
            // create some threads and store them in the vector
        }
    ThreadPool { worker: workers, Sender: sender }
    }

    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let f = Box::new(f);
        self.Sender.send(f);
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 当线程池移出所有权，依次顺序等待所有线程执行完任务
        for worker in &self.worker {
            println!("Shutting down worker: {}", worker.id);

            worker.thread.join().unwrap();
        }    
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job;  executing.", id);
            job()
        });
        Worker { id: id, thread: thread }
    }
}
