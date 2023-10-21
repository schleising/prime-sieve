// Import the num_format crate.
use num_format::{Locale, ToFormattedString};

// Create a structure for the sieve.
struct Sieve {
    // The upper limit of the sieve.
    upper_bound: u64,

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
        // Create a list of booleans of size upperBound.
        let mut initial_candidates = vec![true; upper_bound as usize];

        // Set the first two numbers to false.
        initial_candidates[0] = false;
        initial_candidates[1] = false;

        // Set all multiples of 2 to false.
        for i in (4..upper_bound).step_by(2) {
            initial_candidates[i as usize] = false;
        }

        // Create a new Sieve structure.
        Sieve {
            upper_bound,
            start_index: 3,
            stop_index: (upper_bound as f64).sqrt() as u64,
            initial_candidates,
        }
    }

    // Set indexes which are not prime to false
    fn mark_sieve(&mut self) -> Vec<bool> {
        // Create a copy of the sieve.
        let mut sieve: Vec<bool> = self.initial_candidates.clone();

        // Loop through the sieve.
        for i in (self.start_index..self.stop_index).step_by(2) {
            // If the number is prime.
            if sieve[i as usize] {
                // Set all multiples of the current prime number to false.
                for j in (i * i..self.upper_bound).step_by(i as usize) {
                    sieve[j as usize] = false;
                }
            }
        }

        // Return the sieve.
        sieve
    }

}

// The main function.
fn main() {
    // Create a new Sieve structure.
    let mut sieve: Sieve = Sieve::new(10000000);

    // Run for 5 seconds.
    let now = std::time::Instant::now();

    // Create an empty array for the primes
    let mut primes: Vec<bool> = Vec::new();

    // Initialise a counter for the number of iterations
    let mut iterations: u64 = 0;

    while std::time::Instant::now().duration_since(now).as_secs() < 5 {
        // Mark the sieve.
        primes = sieve.mark_sieve();

        // Increment the number of iterations.
        iterations += 1;
    }

    // Print that this is the Rust version.
    println!("Rust version");

    // Print the number of primes.
    println!("Primes:     {}", primes.iter().filter(|&x| *x).count().to_formatted_string(&Locale::en));

    // Print the number of iterations with commas as thousands separators.
    println!("Iterations: {}", iterations.to_formatted_string(&Locale::en));
}
