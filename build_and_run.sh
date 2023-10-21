#!/bin/zsh

# Build the Go code
go build -ldflags="-w -s" -o ./go/prime-sieve prime-sieve.go

# Build the Rust code
cargo build --release --no-default-features

# Run the Python version
python prime-sieve.py

# Run the Go version
./go/prime-sieve

# Run the Rust version
./target/release/prime-sieve
