use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// This solution is without Barrier<count: usize>

fn main() {
    println!("Main Thread started!");
    
    let num_of_threads = 10;
    let mut threads = Vec::with_capacity(num_of_threads);
    let threads_entered = Arc::new((Mutex::new(0), Condvar::new()));

    for x in 0..num_of_threads {
        let count_clone = threads_entered.clone();
        threads.push(thread::spawn(move || {
            // rendezvous
            let name = format!("Thread {}", x);
            println!("Started {}", name);
            let (count_guard, cvar) = &*count_clone;
            let mut count = count_guard.lock().unwrap();
            
            *count = *count + 1;
            while *count < num_of_threads {
                count = cvar.wait(count).unwrap();
            }

            if *count == num_of_threads {
                cvar.notify_one();
            }

            // critical part
            println!("Critical part DONE by -> {}", name);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
