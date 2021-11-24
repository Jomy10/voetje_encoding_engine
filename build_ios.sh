#!/bin/zsh

# Build file for building the iOS library

echo "Building encoding_engine for iOS target..."
cargo lipo --release

echo "Building C header bridge file..."
cbindgen src/lib.rs -l c > e_engine.h
# cbindgen --config cbindgen.toml --crate encoding_engine --output e_engine.h ## Not working correctly

cd ..

echo "Copying files..."
#cp encoding_engine/e_engine.h include
cp encoding_engine/target/universal/release/libencoding_engine.a libs

# TODO: #4 Rust CLI tool for building
echo "[!] You still have to update the e_engine.h file and run \"cp ../encoding_engine/e_engine.h ../include\""

echo "Build complete."