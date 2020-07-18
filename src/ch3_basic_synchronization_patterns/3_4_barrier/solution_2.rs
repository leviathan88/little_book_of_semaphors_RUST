use std::sync::{Arc, Condvar, Mutex, Barrier};
use std::thread;

// This solution is with Barrier<count: usize>

fn main() {
    println!("Main Thread started!");
    
    let num_of_threads = 10;
    let mut threads = Vec::with_capacity(num_of_threads);
    let barrier = Arc::new(Barrier::new(num_of_threads));

    for x in 0..num_of_threads {
        let barrier_clone = barrier.clone();
        threads.push(thread::spawn(move || {
            // rendezvous
            let name = format!("Thread {}", x);
            println!("Started {}", name);            
            barrier_clone.wait();
            // critical part
            println!("Critical part DONE by -> {}", name);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
