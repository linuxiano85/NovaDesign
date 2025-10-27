# NovaDesign Makefile
# Provides convenient targets for common development tasks

.PHONY: help setup build run fmt clippy test clean flatpak-build flatpak-run flatpak-clean

# Default target
help:
	@echo "NovaDesign Development Makefile"
	@echo "================================"
	@echo ""
	@echo "Available targets:"
	@echo "  help         - Show this help message"
	@echo "  setup        - Run cross-distro development setup"
	@echo "  build        - Build the project in release mode"
	@echo "  run          - Run the nova-app application"
	@echo "  fmt          - Format the code using rustfmt"
	@echo "  clippy       - Run clippy linter"
	@echo "  test         - Run tests"
	@echo "  clean        - Clean build artifacts"
	@echo "  flatpak-build - Build Flatpak package"
	@echo "  flatpak-run  - Run application via Flatpak"
	@echo "  flatpak-clean - Clean Flatpak build artifacts"
	@echo ""
	@echo "For first-time setup, run: make setup"

# Run the cross-distro setup script
setup:
	@echo "Running cross-distro development setup..."
	./scripts/dev-setup.sh

# Build the project in release mode
build:
	cargo build --release

# Run the nova-app application
run:
	cargo run -p nova-app

# Format the code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Build Flatpak package
flatpak-build:
	@echo "Building Flatpak package..."
	@if [ ! -d "build" ]; then mkdir build; fi
	flatpak-builder build flatpak/io.nova.Design.json --force-clean --install-deps-from=flathub

# Run application via Flatpak (requires flatpak-build first)
flatpak-run:
	@echo "Running NovaDesign via Flatpak..."
	flatpak run io.nova.Design

# Clean Flatpak build artifacts
flatpak-clean:
	@echo "Cleaning Flatpak build artifacts..."
	rm -rf build .flatpak-builder

# Install Flatpak application locally
flatpak-install: flatpak-build
	@echo "Installing Flatpak application locally..."
	flatpak-builder --user --install --force-clean build flatpak/io.nova.Design.json

# Development convenience target - build and run
dev: build run