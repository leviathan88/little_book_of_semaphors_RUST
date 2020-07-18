#![feature(get_mut_unchecked)]

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    println!("Main Thread started!");
    
    let count = Arc::new(0);

    let max_visitors = 5;

    let semaphore = Arc::new((
        Mutex::new(max_visitors),
        Condvar::new(),
    ));
    
    let num_of_threads = 10;
    let mut threads = Vec::with_capacity(num_of_threads);

    for x in 0..num_of_threads {
        let sem_clone = semaphore.clone();
        let sem_clone_2 = sem_clone.clone();
        let count_clone = count.clone();

        threads.push(thread::spawn(move || {
            let name = &format!("Thread {}", x);
            increment(sem_clone, count_clone, name);
            decrement(sem_clone_2);
            println!("DONE {}", name);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Final Count is {}", count);
}


fn increment(semaphore: Arc<(Mutex<u16>, Condvar)>, mut count: Arc<i32>, name: &str) {
    println!("{} started", name);
    let (lock, cvar) = &*semaphore;
    let mut visitors_guard = lock.lock().unwrap();

    while *visitors_guard <= 0 {
        println!("{} is waiting now on its turn", name);
        visitors_guard = cvar.wait(visitors_guard).unwrap();
    }

    println!("{} finalizing the process", name);

    *visitors_guard = *visitors_guard - 1;

    unsafe {
        let k = Arc::get_mut_unchecked(&mut count);
        *k = *k + 1;
    }
}

fn decrement(semaphore: Arc<(Mutex<u16>, Condvar)>) {
    add_latency();
    let (lock, cvar) = &*semaphore;
    let mut started = lock.lock().unwrap();

    *started = *started + 1;
    cvar.notify_one();
}

fn add_latency() {
    for _ in 0..10_000_000 {}
}