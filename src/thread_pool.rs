use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle}
};

type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(limit: usize) -> Self {
        assert!(limit > 0);

        let mut workers: Vec<Worker> = Vec::with_capacity(limit);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for id in 0..limit {
            workers.push(Worker::new(id, Arc::clone(&rx)))
        }

        ThreadPool { workers, tx: Some(tx) }
    }

    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.tx.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = rx.lock().unwrap().recv();

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
        
        Worker { id, thread: Some(thread) }
    }
}
