use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v_clone = v.clone();

    let thread1 = std::thread::spawn(move || {
        let mut v_locked = v_clone.lock().unwrap();
        *v_locked = vec![10,11,12];
    });

    let mut v_locked = v.lock().unwrap();
    println!("Before modification: {:?}", *v_locked);
    *v_locked = vec![4,5,6]; 
    println!("After modification: {:?}", *v_locked);
    thread1.join().unwrap();
    let v_locked = v.lock().unwrap();
    println!("Final Vector: {:?}", *v_locked);
} 