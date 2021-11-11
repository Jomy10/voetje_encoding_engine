#!/bin/zsh

# Build file for building the iOS library

echo "Building encoding_engine for iOS target..."
cargo lipo --release

echo "Building C header bridge file..."
#cbindgen src/lib.rs -l c > e_engine.h
cbindgen --config cbindgen.toml --crate src/lib.rs --output e_engine.h

cd ..

echo "Copying files..."
cp encoding_engine/e_engine.h include
cp encoding_engine/target/universal/release/libencoding_engine.a libs

echo "Build complete."