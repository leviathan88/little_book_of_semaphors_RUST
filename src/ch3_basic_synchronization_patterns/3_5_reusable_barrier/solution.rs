use std::sync::{Arc, Condvar, Mutex, Barrier};
use std::thread;

struct ReusableBarrier {
    barrier_1: Arc<Barrier>,
    barrier_2: Arc<Barrier>   
}

impl ReusableBarrier {
    pub fn new(size: usize) -> Self {
        ReusableBarrier {
            barrier_1: Arc::new(Barrier::new(size)),
            barrier_2: Arc::new(Barrier::new(size)),
        }
    }

    pub fn clone(&self) -> Self {
        ReusableBarrier {
            barrier_1: self.barrier_1.clone(),
            barrier_2: self.barrier_2.clone(),
        }
    }

    pub fn wait_1(&self) {
        self.barrier_1.wait();
    }

    pub fn wait_2(&self) {
        self.barrier_2.wait();
    }
}


fn main() {
    println!("Main Thread started!");
    
    let num_of_threads = 10;
    let mut threads = Vec::with_capacity(num_of_threads);    
    let res_bar = ReusableBarrier::new(num_of_threads);

    for x in 0..num_of_threads {
        let res_bar_clone = res_bar.clone();
        threads.push(thread::spawn(move || {
            // rendezvous
            let name = format!("Thread {}", x);
            println!("Started {}", name);            
            res_bar_clone.wait_1();
            // critical part
            println!("Critical part DONE by -> {}", name);

            res_bar_clone.wait_2();
            println!("Second critical part DONE by -> {}", name);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
