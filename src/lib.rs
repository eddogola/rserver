use std::thread;
use std::sync::mpsc; // fifo queue providing message-based communication over channels
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job: executing...", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} told to terminate", id);
                    break;
                },
            }
        }));

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

// type alias the trait object
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Gracefully shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() { // this way ensures no panicking happens if None is found
                thread.join().unwrap();
            }
        }
    }
}