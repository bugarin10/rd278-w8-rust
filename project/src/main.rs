
extern crate chrono;


use chrono::prelude::*;

fn main() {
    // Start measuring memory and time before the loop
    let start_time = Utc::now();

    let mut list1: Vec<i32> = Vec::new();

    // Your for loop here
    for i in 0..1000 {
        for j in 0..1000 {
            for k in 0..1000 {
                list1.push(i + j + k);
            }
        }
    }

    // Measure memory and time after the loop

    let end_time = Utc::now();

    // Calculate memory usage and time taken

    let time_taken = end_time.signed_duration_since(start_time).num_seconds() as f64;


    println!("Time taken: {:.2} seconds",time_taken);
}
