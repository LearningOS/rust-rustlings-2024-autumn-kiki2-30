// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.



use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    
    // Spawn 10 threads
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250)); // Each thread sleeps for 250ms
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // Return the elapsed time in milliseconds
        }));
    }

    // Collect the results from each thread
    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // Join the thread and collect the result
        results.push(handle.join().unwrap()); // `join()` returns the result of the thread
    }

    // Check if all threads finished successfully
    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    // Print the results
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}