
// dont have to run this expensive task on the single core my machine will burn out 
// find the sum of 1 to 10^ 9 
// use the thread to use all the cor of my machine 
use std::{thread};
use std::sync::mpsc;

pub fn multithreadcalculator() {
    let n: u64 = 1_000_000_000; // 1 to 10^9
    let num_threads = 4; // keep it small (safe for laptop)

    let chunk_size = n / num_threads;

    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx_clone = tx.clone();

        let start = i * chunk_size + 1;
        let end = if i == num_threads - 1 {
            n
        } else {
            (i + 1) * chunk_size
        };

        thread::spawn(move || {
            let mut sum = 0;

            for num in start..=end {
                sum += num;
            }

            tx_clone.send(sum).unwrap();
        });
    }
    // explicitly destroys the original sender of the channel.
    //     When a value goes out of scope → it is automatically dropped
    // But here, we manually drop it early
    //     As long as at least one sender (tx) exists, the receiver (rx) assumes:
    // 👉 “more messages might still come”
    drop(tx);

    let mut total_sum = 0;

    for partial in rx {
        total_sum += partial;
    }

    println!("Final Sum: {}", total_sum);
}



// pub fn multithreadcalculator(){
//     let (tx , rx ) = mpsc::channel();
//     let tx1 = tx.clone();
//     thread::spawn(move || { 
//         let vec = vec![
//             String::from("name"),String::from("is"), String::from("yash ")
//         ];
//         tx1.send(vec);
//     });
//     thread::spawn(move || { 
//         let vec = vec![
//             String::from("last"),String::from("name"),String::from("is"), String::from("kelhe ")
//         ];
//         tx.send(vec);
//     });
//     for value in rx{
//         println!("the value from multithread is  {:?}", value);
//     }    
// }






