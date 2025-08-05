#!/bin/bash

# Security scanning script for Ledger Banking Rust

echo "Running security checks for Ledger Banking Rust..."

# Check if required tools are installed
check_tool() {
    if ! command -v $1 &> /dev/null
    then
        echo "$1 could not be found, installing..."
        $2
    fi
}

check_tool "cargo-audit" "cargo install cargo-audit"
check_tool "cargo-deny" "cargo install cargo-deny"

# 1. Check for known vulnerabilities in dependencies
echo "1. Running cargo audit..."
cargo audit

# 2. Check licenses and banned dependencies
echo "2. Running cargo deny..."
cargo deny check

# 3. Run clippy with security lints (excluding the invalid security lint)
echo "3. Running clippy with available lints..."
# Note: clippy::security is not a valid lint group, so we're using other security-related lints
cargo clippy -- -W clippy::suspicious -W clippy::perf -W clippy::complexity

# 4. Check for unused dependencies
echo "4. Checking for unused dependencies..."
cargo install cargo-machete
cargo machete

echo "Security checks completed."