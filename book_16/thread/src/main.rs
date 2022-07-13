use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move || {
        for i in 1..10 {
            println!("Here is a vector: {:?}", v);
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handler.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
