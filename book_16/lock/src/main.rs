use std::{sync::{Mutex, Arc}, thread, rc::Rc};

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:#?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // let counter = counter.clone();
        handles.push(
            thread::spawn(move || {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
                // counter.
            })
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:#?}", counter);
}