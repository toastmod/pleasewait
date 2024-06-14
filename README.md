# pleasewait
My personal implementation of a simple CPU-efficient thread blocker in Rust.

You can set the `std::sync::atomic::Ordering` of the lock flag to your own liking.

## Example
```rust
use std::{sync::{atomic::Ordering, Arc}, time::Duration};
use pleasewait::*;

fn main() 
{
    let waiter = Waiter::create();
    let waiter_clone = Arc::clone(&waiter);

    std::thread::spawn(move || {
        let waiter = waiter_clone;
        println!("Ping!");
        std::thread::sleep(Duration::from_millis(2000));
        waiter.wake(Ordering::SeqCst);
    });

    waiter.wait(Ordering::SeqCst);
    println!("Pong!");
}
```
