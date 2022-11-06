use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 10 {
        let expensive_result = simulated_expensive_calculation(intensity);
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 0 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            let expensive_result = simulated_expensive_calculation(intensity);
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
