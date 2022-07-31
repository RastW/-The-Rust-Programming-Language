use std::{thread, sync::{mpsc, Mutex}};
use std::sync::Arc;

pub struct ThreadPool {
    worker: Vec<Worker>,
    Sender: mpsc::Sender<Job>,
}

// struct Job;
type Job = Box<FnOnce() + Send + 'static>;

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
        self.Sender.send(f)
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job;  executing.", id);
            job()
        });
        Worker { id: id, thread: thread }
    }
}
