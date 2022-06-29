use std::thread;
use std::sync::mpsc; // fifo queue providing message-based communication over channels
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job: executing...", id);

            job()
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

// type alias the trait object
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size); // empty vector of length size

        // create a channel
        let (sender, receiver) = mpsc::channel();
        // put the receiving end in an Arc and a Mutex
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            // create workers and store in workers vector
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        // move channel sender to the threadpool
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}