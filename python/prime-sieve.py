import time


class PrimeSieve:
    def __init__(self, upper_limit: int) -> None:
        # Set the upper limit of the sieve
        self._upper_limit = upper_limit

        # Set the start value for the sieve
        self._start = 3

        # Set the stop value for the sieve
        self._stop = int(self._upper_limit ** 0.5)

        # Initialise an array of integers to store the candidates
        self._initial_candidates = [True] * (self._upper_limit + 1)

        # Set the first two values to zero
        self._initial_candidates[0] = False
        self._initial_candidates[1] = False

        # Set the even numbers to zero
        for number in range(4, self._upper_limit + 1, 2):
            self._initial_candidates[number] = False

    def find_primes(self) -> list[bool]:
        # Create a copy of the initial candidates
        self._candidates = self._initial_candidates.copy()

        # Starting at 3, set the multiples of the current value to zero
        for i in range(self._start,  self._stop + 1, 2):
            if self._candidates[i] == True:
                # Set the multiples of the current value to zero
                for j in range(i * 2, self._upper_limit + 1, i):
                    self._candidates[j] = False

        # Return the candidates
        return self._candidates

    def return_primes(self) -> list[int]:
        # Return the primes
        return [index for index, value in enumerate(self._candidates) if value == True]

if __name__ == '__main__':
    # Initialise a variable to store the upper limit
    upper_limit = 10000000

    # Print that this is the Python version and the upper limit
    print('Python version')
    print(f'Upper bound: {upper_limit:,}')

    # Create a new prime sieve
    prime_sieve = PrimeSieve(upper_limit)

    # Find the primes
    # prime_sieve.find_primes()

    # Create a list of primes
    # primes = prime_sieve.return_primes()

    # Print the primes
    # print(primes)

    # Run for five seconds and count the number of iterations
    iterations = 0
    start_time = time.perf_counter()

    while time.perf_counter() - start_time < 5:
        # Find the primes
        primes = prime_sieve.find_primes()

        # Increment the number of iterations
        iterations += 1

    # Reduce primes to a list of non-zero values
    primes = prime_sieve.return_primes()

    # Print the number of primes
    print(f'Primes:      {len(primes):,}')

    # Print the number of iterations
    print(f'Iterations:  {iterations:,}')

    # Print a blank line
    print()
