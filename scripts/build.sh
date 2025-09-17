#!/bin/bash

# Build in release mode
cargo build --release

# Create bin directory if it doesn't exist
mkdir -p ./bin

# Copy only the binary to bin directory
rm -f ./bin/codecho
cp target/release/codecho ./bin/

echo "Binary copied to ./bin/codecho"