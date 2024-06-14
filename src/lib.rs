use std::{sync::{atomic::Ordering}, thread::Thread};
    
mod waiter;
pub use waiter::*;

#[cfg(test)]
mod tests {
    use std::{sync::{atomic::Ordering, Arc}};
    use super::*;

    #[test]
    fn waiter_works() 
    {
        let waiter = Waiter::create();
        let waiter_clone = Arc::clone(&waiter);

        std::thread::spawn(move || {
            let waiter = waiter_clone;
            waiter.wake();
        });

        waiter.wait();

    }
}
