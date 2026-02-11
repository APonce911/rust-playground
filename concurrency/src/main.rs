use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex (mutual exclusion): guard data with locks
    let mutex = Mutex::new(0);

    // RC (reference-counted) value: Share value ownership(see cap 15)
    // but RC is not thread safe a change to count can be interrupted by other thread
    // let counter = Rc::new(m); // wont work

    // use Arc(Atomic reference-counted)
    let counter = Arc::new(mutex);

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // type mutex<i32>
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
