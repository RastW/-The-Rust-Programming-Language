use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // send将会移交所有权
        tx.send(val).unwrap();
    });

    // 取得所有权
    let received =  rx.recv().unwrap();
    println!("Get: {}", received);

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("2: from"),
            String::from("3: the"),
            String::from("4: thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for recived in rx {
        println!("Got: {}", recived);
    }

    println!("Hello, world!");
}
