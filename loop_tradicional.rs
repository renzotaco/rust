use std::thread;
use std::time::Duration;

fn main() {

    for i in 0..5 {

        println!("Loop 1 iteration: {}", i);
        // wait a bit before next iteration
        // for demonstration purposes
        thread::sleep(Duration::from_millis(500));
    }

    for i in 0..5 {

        println!("Loop 2 iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    }
}
