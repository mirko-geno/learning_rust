use std::{
    net::TcpStream,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle}
};

type Job = Box<dyn FnOnce() + Send + 'static>;


struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>
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

        ThreadPool { workers, tx }
    }

    pub fn _execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.tx.send(job).unwrap();
    }
}


struct Worker {
    id: usize,
    thread: JoinHandle<()>,
    rx: Arc<Mutex<mpsc::Receiver<Job>>>
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {}),
            rx
        }
    }
}



/*
#[allow(dead_code)]
enum State {
    Available(JoinHandle<()>),
    Working(JoinHandle<()>)
}

impl State {
    fn join(self) {
        match self {
            State::Available(h) | State::Working(h) => {
                h.join().unwrap()
            }
        }
    }
}


pub struct ThreadPool {
    threads: Vec<State>,
    queue: Vec<TcpStream>
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(quant: usize) -> Self {
        assert!(quant > 0);
        let mut threads = Vec::new();

        for _ in 0..quant {
            threads.push(State::Available(thread::spawn(
                || println!("Nothing")
            )));
        }

        ThreadPool {
            threads,
            queue: Vec::new()
        }
    }

    pub fn add_to_queue(&mut self, stream: TcpStream) {
        let length = self.queue.len();
        self.queue.insert(length,stream);
    }

    pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static {
        
    } 

    pub fn shutdown(&mut self) {
        let quant = self.threads.len();
        for _ in 0..quant {
            self.threads.pop().unwrap().join();
        }
    }

}
*/