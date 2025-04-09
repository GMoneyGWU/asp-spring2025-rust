#!/bin/bash

# Script to generate the Flutter-Rust bridge code

set -e  # Exit on error

echo "Generating Flutter-Rust bridge code..."

# Check if flutter_rust_bridge_codegen is installed
if ! command -v flutter_rust_bridge_codegen &> /dev/null; then
    echo "flutter_rust_bridge_codegen not found. Installing..."
    cargo install flutter_rust_bridge_codegen
fi

# Generate the bridge code
echo "Running code generation..."
flutter_rust_bridge_codegen --rust-input backend/src/api.rs \
                           --dart-output frontend/lib/api/generated/bridge_generated.dart \
                           --c-output frontend/lib/api/generated/bridge_generated.h \
                           --dart-decl-output frontend/lib/api/generated/bridge_definitions.dart

echo "Building Rust library..."
cd backend
cargo build --release
cd ..

echo "Done."
echo "Next steps:"
echo "1. Check the generated code in frontend/lib/api/generated/"
echo "2. Run 'flutter run' to test the integration"