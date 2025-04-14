// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

// Need Mutex for safe mutation across threads
use std::{
    sync::{Arc, Mutex}, // Import Mutex
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    // Wrap the mutable data (the u32 counter) inside a Mutex,
    // then wrap the Mutex in an Arc for shared ownership.
    // We can simplify and just store the u32 directly in the Mutex.
    let status = Arc::new(Mutex::new(0u32)); // Use Mutex<u32> directly

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            // Acquire the lock on the Mutex. `lock()` returns a MutexGuard.
            // .unwrap() handles the potential error if the lock is poisoned (a thread panicked while holding it).
            let mut num = status_shared.lock().unwrap();
            // Mutate the value through the MutexGuard (which dereferences to the inner type).
            *num += 1;
            // The lock is automatically released when `num` (the MutexGuard) goes out of scope here.
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    // To access the final value, we need to lock the mutex again from the main thread.
    // Dereference the MutexGuard to get the value inside.
    println!("Jobs done: {}", *status.lock().unwrap());
}