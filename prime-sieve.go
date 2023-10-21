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
	initialCandidates []int
}

// Create a new prime sieve
func NewPrimeSieve(upperBound int) *PrimeSieve {
	// Print that we are creating a new prime sieve
	println("Creating a new prime sieve with an upper bound of", upperBound)

	// Create a list of candidates where the index is the number and the value is 1 if it is still a candidate and 0 if it is not
	candidates := make([]int, upperBound+1)

	// Set all candidates to 1
	for i := 0; i < len(candidates); i++ {
		candidates[i] = 1
	}

	// Set 0 and 1 to 0
	candidates[0] = 0
	candidates[1] = 0

	// Set all multiples of 2 to 0
	for i := 4; i < len(candidates); i += 2 {
		candidates[i] = 0
	}

	// Create a new prime sieve
	primeSieve := &PrimeSieve{
		upperBound:        upperBound,
		startIndex:        2,
		stopIndex:         int(math.Sqrt(float64(upperBound))),
		initialCandidates: candidates,
	}

	// Return the prime sieve
	return primeSieve
}

// Find all primes
func (primeSieve *PrimeSieve) FindPrimes() []int {
	// Set the current index
	currentIndex := primeSieve.startIndex

	// Set the candidates to be a copy of the initial candidates
	candidates := make([]int, len(primeSieve.initialCandidates))
	copy(candidates, primeSieve.initialCandidates)

	for currentIndex <= primeSieve.stopIndex {
		// If the current index is a candidate
		if candidates[currentIndex] == 1 {
			// Set all multiples of the current index to 0
			for i := currentIndex * 2; i < len(candidates); i += currentIndex {
				candidates[i] = 0
			}
		}

		// Increment the current index
		currentIndex++
	}

	// Return the candidates
	return candidates
}

// Function to turn a list of candidates into a list of primes
func ReturnPrimes(candidates []int) []int {
	// Create a list of primes
	primes := make([]int, 0)

	// Loop through the candidates
	for i := 0; i < len(candidates); i++ {
		// If the current index is a candidate
		if candidates[i] == 1 {
			// Add it to the list of primes
			primes = append(primes, i)
		}
	}

	// Return the list of primes
	return primes
}

// Main function
func main() {
	// Create a new prime sieve
	primeSieve := NewPrimeSieve(10000000)

	// Run the find primes function continuously for 5 seconds counting the number of times it runs
	iterations := 0

	// Start the timer
	startTime := time.Now()

	// Initialise an empty list of primes
	primes := make([]int, 0)

	// Loop until 5 seconds have passed
	for time.Since(startTime).Seconds() < 5 {
		// Find the primes
		primes = primeSieve.FindPrimes()

		// Increment the iterations
		iterations++
	}

	// Print that this is the Go version
	println("Go version")

	// Create a printer
	p := message.NewPrinter(language.English)

	// Print the number of primes
	p.Printf("Primes:     %v\n", len(ReturnPrimes(primes)))

	// Print the number of iterations with a comma as a thousands separator
	p.Printf("Iterations: %v\n", iterations)
}