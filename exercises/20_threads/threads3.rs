use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    // Clone the sender so each thread gets its own handle.
    let tx_clone = tx.clone();

    // Destructure the queue *before* spawning the threads.
    // This consumes `q`, but now `first_half` and `second_half` are separate variables.
    let first_half = q.first_half;
    let second_half = q.second_half;

    // The first thread captures and takes ownership of `first_half` and `tx`.
    thread::spawn(move || {
        for val in first_half { // Now captures `first_half`
            println!("Sending {val:?}");
            tx.send(val).unwrap(); // Use the original tx here
            thread::sleep(Duration::from_millis(250));
        }
        // tx goes out of scope here, dropping one reference to the channel
    });

    // The second thread captures and takes ownership of `second_half` and `tx_clone`.
    thread::spawn(move || {
        for val in second_half { // Now captures `second_half`
            println!("Sending {val:?}");
            tx_clone.send(val).unwrap(); // Use the cloned tx_clone here
            thread::sleep(Duration::from_millis(250));
        }
        // tx_clone goes out of scope here, dropping the second reference
        // The channel closes when all Senders are dropped.
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();
        let queue_len = queue.first_half.len() + queue.second_half.len(); // Get total expected items

        send_tx(queue, tx); // queue is moved into send_tx here

        let mut received = Vec::with_capacity(queue_len);
        // Use `rx.iter().take(queue_len)` or loop until the channel closes
        // The loop `for value in rx` automatically stops when the channel closes
        // (i.e., when both senders `tx` and `tx_clone` are dropped).
        for value in rx {
            received.push(value);
        }

        // Check if we received the expected number of items
        assert_eq!(received.len(), queue_len, "Did not receive all items");

        received.sort(); // Sort because the order of reception is not guaranteed
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}