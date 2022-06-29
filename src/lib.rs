use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {})
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size); // empty vector of length size

        for i in 0..size {
            // create workers and store in workers vector
            workers.push(Worker::new(i));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static, 
    {

    }
}