extern crate psutil;
use std::time::Instant;
use std::mem::size_of;

fn main() {
    // Start measuring memory and time before the loop
    let start_time = Instant::now();
    let initial_memory = get_memory_usage();

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
    let end_time = Instant::now();
    let final_memory = get_memory_usage();

    // Calculate memory usage and time taken
    let memory_usage = final_memory - initial_memory;
    let time_taken = end_time.duration_since(start_time).as_secs_f64();

    println!("Memory usage: {:.2} bytes", memory_usage);
    println!("Time taken: {:.2} seconds", time_taken);
}

fn get_memory_usage() -> usize {
    // Estimate the current process memory usage
    let process_info = psutil::process::Process::new(std::process::id()).unwrap();
    let mem_info = process_info.memory_info().unwrap();
    let rss = mem_info.rss() as usize;
    
    // Calculate the size of the Vec data structure (list1)
    let list1_size = size_of::<Vec<i32>>();
    
    rss + list1_size
}
