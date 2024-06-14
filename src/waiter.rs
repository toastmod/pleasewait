use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::Thread};

pub struct Waiter {
    thread: Thread
}

impl Waiter {

    pub fn wake(&self) {
        self.thread.unpark();
    }

    pub fn wait(&self) {
       std::thread::park();
    }

    pub fn create() -> Arc<Waiter> {
        Arc::new(
            Waiter {
                thread: std::thread::current()
            }
        ) 
    }
}

pub struct WaitLock {
    parked: AtomicBool,
    waiter: Arc<Waiter>
}

impl WaitLock {
    pub fn wake(&self) {
        if self.parked.load(Ordering::Acquire) {
            self.parked.store(false, Ordering::Release);
            self.waiter.wake();
        }
    }

    pub fn wait(&self) {
       self.parked.store(true, Ordering::Release); 
       self.waiter.wait();
    }

    pub fn create() -> Arc<WaitLock> {
        Arc::new(
            WaitLock {
                parked: AtomicBool::new(false),
                waiter: Waiter::create()
            }
        ) 
    }
}

