#!/bin/zsh

# Build file for building the Android library

# Build script
cd /Users/jonaseveraert/AndroidStudioProjects/tVoetje
./gradlew cargoBuild

## Deprecated ##
# Remove recursively
#rm -r target/aarch64-linux-android
#rm -r target/armv7-linux-androideabi
#rm -r target/i686-linux-android

# Build commands
#cargo build --target aarch64-linux-android --release 
#cargo build --target armv7-linux-androideabi --release
#cargo build --target i686-linux-android --release