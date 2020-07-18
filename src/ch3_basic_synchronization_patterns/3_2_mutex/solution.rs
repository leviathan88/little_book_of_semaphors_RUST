#![feature(get_mut_unchecked)]
// Puzzle 3.2: Add semaphores to the following example to enforce mutual exclusion
// to the shared variable count.
// THREAD A: count = count + 1
// THREAD B: count = count + 1

// ! WARNINING: this one is pretty simple in Rust with Arc<Mutex<value>>
// in order to at least to a certain degree simulate the Semaphores
// I decided to mix it up a bit and have the first thread give advantage to the second thread, 
// and then the second thread notify the first thread once it completes
// I went with some "unsafe" code that is only runnable on nighlty build (as of writing it)

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(PartialEq, Debug, Clone, Copy)]
enum SemaphoreState {
    Wait,
    LetOthersDoIt,
    NowYourTurn
}

fn main() {
    println!("Main Thread started!");
    
    let count = Arc::new(0);

    let semaphore = Arc::new((
        Mutex::new(SemaphoreState::Wait),
        Condvar::new(),
    ));

    let a1_copy = semaphore.clone();
    let a1_count = count.clone();
    let a = thread::spawn(move || {
        increment(a1_copy, a1_count, "A Thread");
        println!("Done A");
    });

    let b1_copy = semaphore.clone();
    let b1_count = count.clone();
    let b = thread::spawn(move || {
        increment(b1_copy, b1_count, "B Thread");
        println!("Done B");
    });

    a.join().unwrap();
    b.join().unwrap();

    println!("count is {}", count);
}


fn increment(semaphore: Arc<(Mutex<SemaphoreState>, Condvar)>, mut count: Arc<i32>, name: &str) {
    let (lock, cvar) = &*semaphore;
    let mut started = lock.lock().unwrap();

    match *started {
        SemaphoreState::Wait => {
            println!("{} giving advantage to the other thread", name);
            *started = SemaphoreState::LetOthersDoIt;

            while *started != SemaphoreState::NowYourTurn {
                started = cvar.wait(started).unwrap();
            }

            // SAFETY: this will only be accesible once the other thread is done incrementing the count
            unsafe {
                let k = Arc::get_mut_unchecked(&mut count);
                *k = *k + 1;
            }

        },
        SemaphoreState::LetOthersDoIt => {
            println!("{} will do the work first and give it back", name);

            // SAFETY: since the first thread gave us the right to increment first, 
            // it will wait for us to increment the count and once we're done we will inform it
            unsafe {
                let k = Arc::get_mut_unchecked(&mut count);
                *k = *k + 1;
            }

            *started = SemaphoreState::NowYourTurn;
            cvar.notify_one();
        },
        SemaphoreState::NowYourTurn => { unreachable!(); }
    };
}