use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(PartialEq, Debug, Clone, Copy)]
enum SemaphoreState {
    NotReady,
    Ready,
}

fn main() {
    println!("Main Thread started!");
        
    let semaphore = Arc::new((
        // tuple (SemaphoreState,SemaphoreState) -> when (Ready,_) A2 is ready, when (_,Ready) B2 is ready
        Mutex::new((SemaphoreState::NotReady, SemaphoreState::NotReady)),
        Condvar::new(),
    ));

    // Thread A
    let a1_copy = semaphore.clone();
    let a = thread::spawn(move || {
        let (lock, cvar) = &*a1_copy;
        // either first or second
        println!("A1");

        let mut state_guard = lock.lock().unwrap();
        *state_guard = (state_guard.0, SemaphoreState::Ready);
        cvar.notify_one();

        // after B1
        while state_guard.0 != SemaphoreState::Ready {
            state_guard = cvar.wait(state_guard).unwrap();
        }
        println!("A2");
    });

    // Thread B
    let b1_copy = semaphore.clone();
    let b = thread::spawn(move || {
        let (lock, cvar) = &*b1_copy;
        // either first or second
        println!("B1");

        let mut state_guard = lock.lock().unwrap();
        *state_guard = (SemaphoreState::Ready, state_guard.1);
        cvar.notify_one();

        // AFTER A1
        while state_guard.1 != SemaphoreState::Ready {
            state_guard = cvar.wait(state_guard).unwrap();
        }
        println!("B2");
    });

    a.join().unwrap();
    b.join().unwrap();
}
