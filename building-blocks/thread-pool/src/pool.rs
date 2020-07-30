use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

pub struct ThreadPool {
    workers: VecDeque<Worker>, 
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(threads: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut vd = VecDeque::with_capacity(threads as usize);
        let (tx, rx) = mpsc::channel();
        let reciver = Arc::new(Mutex::new(rx));
        for id in 0..threads {
            vd.push_back(Worker::new(id, Arc::clone(&reciver)));
        }
        Ok(ThreadPool {
            workers: vd,
            sender: tx,
        })
    }

    pub fn spawn<F>(&self, job: F) where F: FnBox + Send + 'static {
        let job = Box::new(job);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    let message = receiver.lock().unwrap().recv().unwrap();

                    
                    match message {
                        Message::NewJob(job) => {
                            println!("thread number: {} get a job", id);
                            job.call_box();
                        }
                        Message::Terminate => break,
                    }
                }
            }))
        }
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}