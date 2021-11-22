use std::sync::mpsc;
use std::thread;

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(std::time::Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(std::time::Duration::from_millis(1));
    // }
    //
    // Note that with the above implementation, the new thread will be stopped when
    // the main thread ends, whether or not it has finished running.

    // The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that,
    // when we call the join method on it, will wait for its thread to finish.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    // Calling join on the handle will wait for the thread to finish.
    handle.join().unwrap();


    let v = vec![1, 2, 3];

    // The move keyword tells Rust that we want to take ownership of the vector and
    // pass it to the closure.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // Using Message Passing to Transfer Data Between Threads
    //
    // The mpsc (multiple producer, single consumer) channel type is a channel that
    // allows multiple producers to send data to a single consumer.
    // The abbreviations tx and rx are traditionally used in many fields for transmitter
    // and receiver respectively, so we name our variables as such to indicate each end.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Channels and Ownership Transference

    let (tx1, rx1) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
        // This wouldn't work because the value has already been moved to the other thread.
        // The send function takes ownership of its parameter, and when the value is moved,
        // the receiver takes ownership of it.
        // println!("val is {}", val);
    });

    let received = rx1.recv().unwrap();
    println!("Got: {}", received);

    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // we’re not calling the recv function explicitly anymore: instead, we’re treating rx as
    // an iterator. For each value received, we’re printing it. When the channel is closed,
    // iteration will end.
    for received in rx2 {
        println!("Got: {}", received);
    }

    // Creating Multiple Producers by Cloning the Transmitter
    let (tx3, rx3) = mpsc::channel();

    let tx3_clone = tx3.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx3_clone.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx3 {
        println!("Got: {}", received);
    }
}
