use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::Thread};

pub struct Waiter {
    parked: AtomicBool,
    thread: Thread
}

impl Waiter {

    pub fn wake(&self, order: Ordering) {
        if self.parked.load(order) {
            self.parked.store(false, order);
            self.thread.unpark();
        }
    }

    pub fn wait(&self, order: Ordering) {
       self.parked.store(true, order); 
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

