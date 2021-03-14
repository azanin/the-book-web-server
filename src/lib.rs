use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use crate::Message::Terminate;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct PoolCreationError {
    message: String
}

impl PoolCreationError {
    fn new(message: &str) -> PoolCreationError {
        PoolCreationError { message: String::from(message) }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size).map(|id| Worker::new(id, Arc::clone(&receiver))).collect();

        println!("Created ThreadPool");

        ThreadPool { workers, sender }
    }

    /*    pub fn new2<'a>(size: usize) -> Result<ThreadPool, PoolCreationError> {
            if size == 0 { Err(PoolCreationError::new("size can't be 0")) } else {
                let workers: Vec<Worker> = (0..size).map(|id| Worker::new(id)).collect();
                Ok(ThreadPool { workers })
            }
        }*/

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        self.workers.iter().for_each(|_| self.sender.send(Terminate).unwrap());

        self.workers.iter_mut().for_each(|w| {
            println!("Shutting down worker {}", w.id);
            if let Some(t) = w.thread.take() {
                t.join().unwrap()
            }
        })
    }
}