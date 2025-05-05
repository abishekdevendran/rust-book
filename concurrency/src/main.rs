use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{sleep, spawn}, time::Duration,
};

fn main() {
    // let (tx, rx) = mpsc::channel();
    // let tx2 = tx.clone();
    // spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    // spawn(move || {
    //     let val = String::from("hi2");
    //     tx2.send(val).unwrap();
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    let counter = Arc::new(Mutex::new(0)); // Mutex::new(5);
    let mut handles = vec![];
    for i in 0..10 {
        let counter = counter.clone();
        let handle = spawn(move || {
            println!("hi number {}", i);
            sleep(Duration::from_millis(50*i/250));
            let mut num = counter.lock().unwrap();
            *num += i;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter={}", counter.lock().unwrap());
}
