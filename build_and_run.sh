# Remove the old executables
rm -rf ./cpp/C++
rm -rf ./go/go
rm -rf ./rust/target

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

# Run the Python version
python ./python/prime-sieve.py

# Run the C++ version
./cpp/C++/prime-sieve

# Run the Go version
./go/go/prime-sieve

# Run the Rust version
./rust/target/release/prime-sieve
