use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

pub fn run() {

    // (I) Creating and Joining threads
    let mut thread_handles = vec![];

    for i in 0..10 {

        // `move` moves the ownership of i to the thread
        // this guarentees that i will not be modified outside of the thread
        thread_handles.push(thread::spawn(move || {
            println!("thread #{}", i);
        }));

        // this will not work since the main thread does not own i anymore
        // Compiler will throw a 'use of moved value' error
        // i -= 1
    }

    for thread in thread_handles {
        thread.join().unwrap();
    }



    // (II) Channels

    let (transmit_end, recieve_end) = mpsc::channel();

    thread::spawn(move || {
        transmit_end.send(42).unwrap(); 
    });

    // blocking recv
    println!("recieved {}", recieve_end.recv().unwrap());

    // nonblocking recv
    assert!(recieve_end.try_recv().is_err());



    /* (III) Mutexes and Arcs */

    // mutex with an int* initialized to 0
    let data = Arc::new(Mutex::new(0));
    let mut thread_handles_2 = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let thread = thread::spawn(move || {
            // gets automatically unlocked when the thread goes out of scope
            let mut data = data.lock().unwrap();
            *data += 1;
        });
        thread_handles_2.push(thread);
    }

    for thread in thread_handles_2 {
        thread.join().unwrap();
    }

    println!("num: {}", *(data.lock().unwrap()));
}

