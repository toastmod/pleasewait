# pleasewait

My personal implementation of a simple CPU-efficient thread blocker in Rust.

You can set the `std::sync::atomic::Ordering` of the lock flag to your own liking.

## Example

```rust
use std::{sync::Arc, time::Duration};
use pleasewait::*;
fn main()
{
    let waiter = Waiter::create();
    let waiter_clone = Arc::clone(&waiter);

    std::thread::spawn(move || {
        let waiter = waiter_clone;
        loop {
            print!("Ping!");
            std::thread::sleep(Duration::from_millis(400));
            waiter.wake();
        }
    });

    loop {
        waiter.wait();
        println!("Pong!");
        std::thread::sleep(Duration::from_millis(1000));
    }
}
```
