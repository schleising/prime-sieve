#! /bin/zsh

# Build the Rust code
cargo build --release --no-default-features

# Build the Go code
go build -ldflags="-w -s" -o ./go/prime-sieve prime-sieve.go

# Run the Python version
python prime-sieve.py

# Run the Rust version
./target/release/prime-sieve

# Run the Go version
./go/prime-sieve
