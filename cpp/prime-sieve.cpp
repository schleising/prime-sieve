// Create a class PrimeSieve that contains a vector of bools. The vector represents the sieve of Eratosthenes.

#include <iostream>
#include <vector>
#include <cmath>

using namespace std;

class PrimeSieve {
public:
    PrimeSieve(int n) {
        // Set the upper bound
        upper_bound = n;

        // Set the start and end indices
        start_index = 3;
        end_index = sqrt(n) + 1;

        for (int i = 0; i < n; i++) {
            sieve.push_back(true);
        }
        sieve[0] = false;
        sieve[1] = false;
        for (int i = 4; i < sieve.size(); i += 2) {
            sieve[i] = false;
        }
    }

    // Go through the sieve and cross out all multiples of each prime
    vector<bool> sieveOfEratosthenes() {
        // Make a copy of the sieve
        vector<bool> copy = sieve;

        // Go through the sieve and cross out all multiples of each prime
        for (int i = start_index; i < end_index; i += 2) {
            if (copy[i]) {
                for (int j = i * i; j < copy.size(); j += i * 2) {
                    copy[j] = false;
                }
            }
        }

        return copy;
    }

private:
    int upper_bound;
    int start_index;
    int end_index;
    vector<bool> sieve;
};

// Main function
int main() {
    // Set the upper bound of the sieve
    int upper_bound = 10000000;

    // Print that this is the C++ version and the upper bound
    cout << "C++ version" << endl << endl;
    cout << "Upper bound: " << upper_bound << endl << endl;

    PrimeSieve sieve(upper_bound);

    // Initialise an empty vector to store the primes
    vector<bool> primes;

    // Initialise a count of iterations
    int iterations = 0;

    // Run continuously for 5 seconds
    clock_t start = clock();

    while ((clock() - start) / CLOCKS_PER_SEC < 5) {
        // Get the sieve of Eratosthenes
        primes = sieve.sieveOfEratosthenes();

        // Increment the count of iterations
        iterations++;
    }

    // Count the number of primes
    int count = 0;
    for (int i = 0; i < primes.size(); i++) {
        if (primes[i]) {
            count++;
        }
    }

    // Print the number of primes
    cout << "Primes:      " << count << endl << endl;

    // Print the number of iterations
    cout << "Iterations:  " << iterations << endl << endl;

    // Print a blank line
    cout << endl << endl;

    return 0;
}
