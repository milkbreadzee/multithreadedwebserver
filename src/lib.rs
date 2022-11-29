use std::{
    sync::{ mpsc, Arc, Mutex },
    thread,
};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().expect("error...").recv().expect("thread holding sender shut down.");

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

//The Mutex<T> ensures that only one Worker thread at a time is trying to request a job.

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0); //bc a pool with no threads make no sense 

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }

       
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).expect("hi1");
    }
}
