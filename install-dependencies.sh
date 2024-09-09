#!/usr/bin/env bash

# Function to check if Homebrew is installed
check_brew_installed() {
    if command -v brew &>/dev/null; then
        echo "Homebrew is already installed."
    else
        echo "Homebrew is not installed. Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        echo "Homebrew installed successfully."
    fi
}

# Function to check if Rust is installed
check_rust_installed() {
    if brew list rust &>/dev/null; then
        echo "Rust is already installed."
    else
        echo "Rust is not installed. Installing Rust using Homebrew..."
        brew install rust
        echo "Rust installed successfully."
    fi
}

# Main script
check_brew_installed
check_rust_installed
