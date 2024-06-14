use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::Thread};

pub struct Waiter {
    parked: AtomicBool,
    thread: Thread
}

impl Waiter {

    pub fn wake(&self) {
        if self.parked.load(Ordering::Acquire) {
            self.parked.store(false, Ordering::Release);
            self.thread.unpark();
        }
    }

    pub fn wait(&self) {
       self.parked.store(true, Ordering::Release); 
       std::thread::park();
    }

    pub fn create() -> Arc<Waiter> {
        Arc::new(
            Waiter {
                parked: AtomicBool::new(false),
                thread: std::thread::current()
            }
        ) 
    }
}

