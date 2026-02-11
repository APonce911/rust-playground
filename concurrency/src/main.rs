use std::sync::Mutex;

fn main() {
    // mutual exclusion 
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        // type mutex<i32>
        *num = 6;
    }

    println!("m = {m:?}");
}
