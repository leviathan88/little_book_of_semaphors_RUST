# Little Book of Semaphores - RUST Edition

```
This Repo contains Rust solutions for the Puzzles from the
"*Little Book of Semaphores*"
```

### Approach
```
Rust language does not have built-in Semaphores, at least not anymore.
My approach was to use what Rust offers for concurrent programming and that is:

- Arc<T>
- Mutex<T>
- Condvar<T>
- Barrier

Having said that, in same cases, using these structures and Rust 
best practices felt like cheating, so in order to build something
that is close to a **Semaphore** I had to resort to more convoluted 
solutions and using "unsafe" code.

```

### Signal & Wait
```
In order to achieve the Semaphore-like behavior
of _signal()_ and _wait()_ I have used Condvar Struct,

Documentation: "Condition variables represent the ability to block a thread such that it consumes no CPU time while waiting for an event to occur."

Also, to be able to put a thread to sleep, while the other thread is doing the work, and holding the _lock_ on the Mutex, I added some aritficial latency,
so that the thread actually goes to sleep and gets awake via Condvar once 
the state has changed (_the signal_), and not once the Mutex lock is released.
```

### How to run and test the output
```
Choose which Puzzle you want to run,
copy/paste the code into **main.rs** and run

$ cargo run

I decided sometimes not to use the Mutex inside the Arc,
in order just to come close to other, less-safe languages,
and in order to use some of the "unsafe" features I had to run _nightly_ build,
so if the code breaks, please switch to the nighlty by running

$ rustup default nightly
```