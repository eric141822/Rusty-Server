use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: u32,
    _thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(_id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let _thread = thread::spawn(move || loop {

            // 'recv' returns error if the sender is dropped.
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", _id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} shutting down.", _id);
                    break;
                }
            }
        });
        Worker {id: _id, _thread: Some(_thread)}
    }
}

pub struct PoolCreationError;

impl std::fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pool Creation Error: Size cannot be 0.")
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(_size: u32) -> ThreadPool {
        assert!(_size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(_size as usize);
        for _id in 0.._size {
            workers.push(Worker::new(_id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender: Some(sender)}
    }
    
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Errors
    /// 
    /// The `build` function will return an error if the size is zero.
    pub fn build(_size: u32) -> Result<ThreadPool, PoolCreationError> {
        if _size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(_size as usize);
            for _id in 0.._size {
                workers.push(Worker::new(_id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool {workers, sender: Some(sender)})
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(_f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for _worker in &mut self.workers {
            println!("Dropping worker {}", _worker.id);
            if let Some(_thread) = _worker._thread.take() {
                _thread.join().unwrap();
            }
        }
    }
}