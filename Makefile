# Makefile for Unreal Engine Log Parser

.PHONY: build run test fmt clippy clean help

# Default target
all: build

# Build the project in release mode
build:
	@echo "Building the project..."
	cargo build --release

# Run the project
run:
	@echo "Running the project..."
	cargo run --release

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Format the code
fmt:
	@echo "Formatting the code..."
	cargo fmt

# Lint the code with Clippy
clippy:
	@echo "Linting the code with Clippy..."
	cargo clippy -- -D warnings

# Clean the project
clean:
	@echo "Cleaning the project..."
	cargo clean

# Display help
help:
	@echo "Available commands:"
	@echo "  make build    - Build the project in release mode"
	@echo "  make run      - Run the project"
	@echo "  make test     - Run tests"
	@echo "  make fmt      - Format the code"
	@echo "  make clippy   - Lint the code with Clippy"
	@echo "  make clean    - Clean the project"
	@echo "  make help     - Show this help message"
