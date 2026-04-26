use std::{thread, time::Duration};
use std::sync::mpsc;
// multi-producer, single-consumer (mpsc)


pub fn implthread() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("print numbers {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // join() blocks the current thread (main thread)
    // until the spawned thread has finished executing.
    // It DOES NOT mean the spawned thread runs first.
    // Both threads run concurrently, but when main reaches join(),
    // it waits for the spawned thread to complete.

    handle.join().unwrap();

    // This loop will only run AFTER the spawned thread is done
    for i in 0..4 {
        println!("numbers are {}", i);
        thread::sleep(Duration::from_millis(1));
    }


    // clousres 
    // to solve the dangling pointer issue we use the "move" keyword to give the owner ship to the thread 

    // example 
    {
        let vec = vec![12,2,2,3];
        // the move means that we have given the owner ship of vec to this spawn thread 
        thread::spawn(move || {
            println!(" the vec is from thread {:?}", vec);
            thread::sleep(Duration::from_millis(1));
        });
    }

    print!("the thread is over ");
}


pub fn talk_to_multiple_thread(){
    let (tx, rx ) = mpsc::channel();


    thread::spawn(move || {
        let value  = String::from("yash kelhe");

        match tx.send(value) {
            Ok(_) => {
                println!("the value has been passed");
            },
            Err(_) => {
                println!("there is error");
            },
        };
    });

    let valuereceived = rx.recv();
// you can use the unwrap here rather then the match but it will stop the program if the error occurs 
    match valuereceived {
        Ok(value)=>{
            println!("the value is from thread |||||||||| {}", value)
        }
        Err(err)=>{
            println!("the error is{} ", err);
        }
    }


}