# Remove the old executables
rm -rf ./cpp/C++
rm -rf ./go/go
rm -rf ./rust/target

rm README.md

# Run the C++ version
mkdir -p ./cpp/C++
g++ -O3 -std=gnu++20 -DNDEBUG ./cpp/prime-sieve.cpp -o ./cpp/C++/prime-sieve

# Change to the Go directory
cd ./go

# Build the Go code
go build -ldflags="-w -s" -o ./go/prime-sieve ./prime-sieve.go

# Change back to the root directory
cd ..

# Change to the Rust directory
cd ./rust

# Build the Rust code
cargo build --release --no-default-features

# Change back to the root directory
cd ..

# Generate the README.md file
echo "[![Build and Run](https://github.com/schleising/prime-sieve/actions/workflows/build_and_run.yaml/badge.svg)](https://github.com/schleising/prime-sieve/actions/workflows/build_and_run.yaml)" >> README.md
echo "# Prime Sieve Benchmark" >> README.md
echo "" >> README.md
echo "## Python" >> README.md

# Run the Python version
python ./python/prime-sieve.py >> README.md

echo "" >> README.md
echo "## C++" >> README.md

# Run the C++ version
./cpp/C++/prime-sieve >> README.md

echo "" >> README.md
echo "## Go" >> README.md

# Run the Go version
./go/go/prime-sieve >> README.md

echo "" >> README.md
echo "## Rust" >> README.md

# Run the Rust version
./rust/target/release/prime-sieve >> README.md
