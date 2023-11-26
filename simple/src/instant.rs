use std::time::{Duration, Instant};

pub fn sample_test() {
    let start = Instant::now(); // Starts the timer
    println!("Start time: {:?}", start);
    // Some time-consuming operation
    do_something();

    let duration = start.elapsed(); // Gets the elapsed time
    println!("Time taken: {:?}", duration);
}

fn do_something() {
    // simulate a time-consuming task
    std::thread::sleep(Duration::from_secs(2));
}
