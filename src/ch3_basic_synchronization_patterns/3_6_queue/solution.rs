use std::sync::{Arc, Condvar, Mutex, Barrier};
use std::thread;

fn main() {
    println!("Main Thread started!");
    
    let num_of_threads = 5;
    let mut threads = Vec::with_capacity(num_of_threads * 2);

    let semaphore = Arc::new((
        Mutex::new((0, 0)),
        Condvar::new(),
    ));

    let barrier = Arc::new(Barrier::new(num_of_threads * 2));
    
    for x in 0..num_of_threads {
        let name = format!("Leader Thread {}", x);
        let semaphore = semaphore.clone();
        let barrier = barrier.clone();

        threads.push(thread::spawn(move || {
            println!("{} entered the ballroom", name);
            barrier.wait();

            let (lock, cvar) = &*semaphore;
            let mut tracker_guard = lock.lock().unwrap();

            while (*tracker_guard).0 == (*tracker_guard).1 {
                tracker_guard = cvar.wait(tracker_guard).unwrap();
            }

            let tracker = *tracker_guard;
            let (leader_count, follower_count) = tracker;
            *tracker_guard = (leader_count + 1, follower_count);
            cvar.notify_all();

            println!("{} Dances away with it into the sunset", name);
        }));
    }

    for x in 0..num_of_threads {
        let name = format!("Follower Thread {}", x);
        let semaphore = semaphore.clone();
        let barrier = barrier.clone();
        threads.push(thread::spawn(move || {
            println!("{} entered the ballroom", name);
            barrier.wait();

            let (lock, cvar) = &*semaphore;
            let mut tracker_guard = lock.lock().unwrap();
            
            while (*tracker_guard).0 != (*tracker_guard).1 {
                tracker_guard = cvar.wait(tracker_guard).unwrap();
            }

            let tracker = *tracker_guard;
            let (leader_count, follower_count) = tracker;
            *tracker_guard = (leader_count, follower_count + 1);
            cvar.notify_one();

            println!("{} notifies that is ready to dance", name);

        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
