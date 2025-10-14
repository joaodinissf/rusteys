#!/bin/bash
# Helper script to build and run the Rusteys key display overlay

echo "Building Rusteys..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful! Starting Rusteys - Key Display Overlay..."
    ./target/release/rusteys.exe
else
    echo "Build failed. Please check the error messages above."
    exit 1
fi
