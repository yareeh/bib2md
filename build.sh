#!/usr/bin/env bash

# Build the Rust project
echo "Building the bib2md project..."
cargo build --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi
