package main

import (
	"math"
	"time"

	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

type PrimeSieve struct {
	// Upper bound of the sieve
	upperBound int

	// Start of the sieve
	startIndex int

	// End of the sieve
	stopIndex int

	// List of candidates
	initialCandidates []bool
}

// Create a new prime sieve
func NewPrimeSieve(upperBound int) *PrimeSieve {
	// Create a list of candidates where the index is the number and the value is 1 if it is still a candidate and 0 if it is not
	candidates := make([]bool, upperBound+1)

	// Set all candidates to 1
	for i := 0; i < len(candidates); i++ {
		candidates[i] = true
	}

	// Set 0 and 1 to 0
	candidates[0] = false
	candidates[1] = false

	// Set all multiples of 2 to 0
	for i := 4; i < len(candidates); i += 2 {
		candidates[i] = false
	}

	// Create a new prime sieve
	primeSieve := &PrimeSieve{
		upperBound:        upperBound,
		startIndex:        3,
		stopIndex:         int(math.Sqrt(float64(upperBound))) + 1,
		initialCandidates: candidates,
	}

	// Return the prime sieve
	return primeSieve
}

// Find all primes
func (primeSieve *PrimeSieve) FindPrimes() []bool {
	// Set the candidates to be a copy of the initial candidates
	candidates := make([]bool, len(primeSieve.initialCandidates))
	copy(candidates, primeSieve.initialCandidates)

	for i := primeSieve.startIndex; i < primeSieve.stopIndex; i += 2 {
		// If the current index is a candidate
		if candidates[i] {
			// Set all multiples of the current index to 0
			for j := i * i; j < len(candidates); j += i * 2 {
				candidates[j] = false
			}
		}
	}

	// Return the candidates
	return candidates
}

// Function to turn a list of candidates into a list of primes
func ReturnPrimes(candidates []bool) []int {
	// Create a list of primes
	primes := make([]int, 0)

	// Loop through the candidates
	for i := 0; i < len(candidates); i++ {
		// If the current index is a candidate
		if candidates[i] {
			// Add it to the list of primes
			primes = append(primes, i)
		}
	}

	// Return the list of primes
	return primes
}

// Main function
func main() {
	// Set the upper bound
	upperBound := 10000000

	// Print that this is the Go version and the upper bound
	// Create a printer
	p := message.NewPrinter(language.English)
	p.Printf("Go version\n\n")
	p.Printf("Upper bound: %v\n\n", upperBound)

	// Create a new prime sieve
	primeSieve := NewPrimeSieve(upperBound)

	// Run the find primes function continuously for 5 seconds counting the number of times it runs
	iterations := 0

	// Start the timer
	startTime := time.Now()

	// Initialise an empty list of primes
	primes := make([]bool, 0)

	// Loop until 5 seconds have passed
	for time.Since(startTime).Seconds() < 5 {
		// Find the primes
		primes = primeSieve.FindPrimes()

		// Increment the iterations
		iterations++
	}

	// Print the number of primes
	p.Printf("Primes:      %v\n\n", len(ReturnPrimes(primes)))

	// Print the number of iterations with a comma as a thousands separator
	p.Printf("Iterations:  %v\n\n", iterations)

	// Print a blank line
	p.Println()
}
