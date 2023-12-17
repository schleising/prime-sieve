// Import the num_format crate.
use num_format::{Locale, ToFormattedString};

// Import Instant from the standard library.
use std::time::Instant;

// Create a structure for the sieve.
struct Sieve {
    // The starting index of the sieve.
    start_index: u64,

    // The stop index
    stop_index: u64,

    // The sieve itself.
    initial_candidates: Vec<bool>,
}

// Implement the Sieve structure.
impl Sieve {
    // Create a new Sieve structure.
    fn new(upper_bound: u64) -> Sieve {
        // The start index is 3.
        let start_index: u64 = 3u64;

        // The stop index is the square root of the upper bound.
        let stop_index: u64 = (upper_bound as f64).sqrt() as u64 + 1u64;

        // Print the stop index.
        println!("Stop index    : {}", stop_index.to_formatted_string(&Locale::en));
        println!();

        // Create a list of booleans of size upperBound.
        let mut initial_candidates = vec![true; upper_bound as usize];

        // Set the first two numbers to false.
        initial_candidates[0] = false;
        initial_candidates[1] = false;

        // Set all multiples of 2 to false.
        for i in (start_index + 1u64..upper_bound).step_by(2usize) {
            initial_candidates[i as usize] = false;
        }


        // Create a new Sieve structure.
        Sieve {
            start_index,
            stop_index,
            initial_candidates,
        }
    }

    // Set indexes which are not prime to false
    fn mark_sieve_iterator(&self) -> Vec<bool> {
        // Create a copy of the sieve.
        let mut sieve: Vec<bool> = self.initial_candidates.clone();

        // Loop through the sieve.
        for i in (self.start_index..self.stop_index).step_by(2) {
            #[cfg(debug_assertions)]
            println!("---------------------------");
            #[cfg(debug_assertions)]
            println!("i             : {}", i);

            // If the number is prime.
            if sieve[i as usize] {
                // Set all multiples of the number to false using an iterator.
                sieve.iter_mut()
                    .skip((i * i) as usize)
                    .step_by((i * 2) as usize)
                    .for_each(|x| {
                        *x = false
                    });
            }
        }

        // Return the sieve.
        sieve
    }

    fn mark_sieve_loop(&self) -> Vec<bool> {
        // Create a copy of the sieve.
        let mut sieve: Vec<bool> = self.initial_candidates.clone();

        // Loop through the sieve.
        for i in (self.start_index..self.stop_index).step_by(2) {
            #[cfg(debug_assertions)]
            println!("---------------------------");
            #[cfg(debug_assertions)]
            println!("i             : {}", i);

            // If the number is prime.
            if sieve[i as usize] {
                // Set all multiples of the number to false using a loop.
                let mut j: u64 = i * i;
                while j < sieve.len() as u64 {
                    sieve[j as usize] = false;
                    j += i * 2;
                }
            }
        }

        // Return the sieve.
        sieve
    }

}

// The main function.
fn main() {
    // Set the upper bound.
    let upper_bound: u64 = 10000000;

    // Print that this is the Rust version and the upper bound.
    println!("Rust version");
    println!();
    println!("Upper bound   : {}", upper_bound.to_formatted_string(&Locale::en));
    println!();

    // Create a new Sieve structure.
    let sieve: Sieve = Sieve::new(upper_bound);

    // Run for 5 seconds.
    let start: Instant = Instant::now();

    // Create an empty array for the primes
    let mut primes: Vec<bool> = Vec::new();

    // Initialise a counter for the number of iterations
    let mut iterations: u64 = 0;

    // Run the iterator veriosn for 5 seconds.
    while Instant::now().duration_since(start).as_secs() < 5 {
        // Mark the sieve.
        primes = sieve.mark_sieve_iterator();

        // Increment the number of iterations.
        iterations += 1;
    }

    // Print that this is the iterator version.
    println!("### Iterator version");
    println!();

    // Print the number of primes.
    println!("Primes        : {}", primes.iter().filter(|&x| *x).count().to_formatted_string(&Locale::en));
    println!();

    // Print the prime numbers on a single line.
    if cfg!(debug_assertions) {
        print!("Prime nums    : ");
        primes.iter().enumerate().for_each(|(i, x)| {
            if *x {
                print!("{} ", i);
            }
        });
        println!();
    }

    // Print the number of iterations with commas as thousands separators.
    println!("Iterations    : {}", iterations.to_formatted_string(&Locale::en));
    println!();

    // Reset the start time and iterations.
    let start: Instant = Instant::now();
    iterations = 0;

    // Run the loop version for 5 seconds.
    while Instant::now().duration_since(start).as_secs() < 5 {
        // Mark the sieve.
        primes = sieve.mark_sieve_loop();

        // Increment the number of iterations.
        iterations += 1;
    }

    // Print that this is the loop version.
    println!("### Loop version");
    println!();

    // Print the number of primes.
    println!("Primes        : {}", primes.iter().filter(|&x| *x).count().to_formatted_string(&Locale::en));
    println!();

    // Print the prime numbers on a single line.
    if cfg!(debug_assertions) {
        print!("Prime nums    : ");
        primes.iter().enumerate().for_each(|(i, x)| {
            if *x {
                print!("{} ", i);
            }
        });
        println!();
    }

    // Print the number of iterations with commas as thousands separators.
    println!("Iterations    : {}", iterations.to_formatted_string(&Locale::en));
    println!();

    // Print a newline.
    println!();
    println!();
}
