#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status.

# Function to check if a command was successful
check_success() {
    if [ $? -ne 0 ]; then
        echo "Error: $1 failed"
        exit 1
    fi
}

# Build for all Android targets
for target in aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
do
    echo "Building for $target"
    cargo build --target $target --release
    check_success "Building for $target"
done

# Generate Kotlin bindings
echo "Generating Kotlin bindings"
cargo run --bin uniffi-bindgen generate src/lib.udl --language kotlin
check_success "Generating Kotlin bindings"

# Create directories for the AAR structure
mkdir -p aar/jni/arm64-v8a aar/jni/armeabi-v7a aar/jni/x86 aar/jni/x86_64

# Copy the compiled libraries to the appropriate directories
echo "Copying compiled libraries"
cp target/aarch64-linux-android/release/libChemistryEquationsBackbone.so aar/jni/arm64-v8a/
cp target/armv7-linux-androideabi/release/libChemistryEquationsBackbone.so aar/jni/armeabi-v7a/
cp target/i686-linux-android/release/libChemistryEquationsBackbone.so aar/jni/x86/
cp target/x86_64-linux-android/release/libChemistryEquationsBackbone.so aar/jni/x86_64/
check_success "Copying compiled libraries"

# Copy Kotlin bindings
echo "Copying Kotlin bindings"
cp -r src/uniffi/ChemistryEquationsBackbone aar/java
check_success "Copying Kotlin bindings"

# Create the AAR
echo "Creating AAR file"
cd aar
zip -r ../ChemistryEquationsBackbone.aar *
check_success "Creating AAR file"
cd ..

echo "AAR file created: ChemistryEquationsBackbone.aar"

# Second Attempt

# cargo build --lib \
#     --target x86_64-linux-android \
#     --target i686-linux-android \
#     --target armv7-linux-androideabi \
#     --target aarch64-linux-android
# check_success "Building for Android"

# mkdir -p jniLibs/arm64-v8a/ && \
#   cp target/aarch64-linux-android/debug/libChemistryEquationsBackbone.so jniLibs/arm64-v8a/libuniffi_chemistryequationsbackbone.so && \
#   mkdir -p jniLibs/armeabi-v7a/ && \
#     cp target/armv7-linux-androideabi/debug/libChemistryEquationsBackbone.so jniLibs/armeabi-v7a/libuniffi_chemistryequationsbackbone.so && \
#   mkdir -p jniLibs/x86/ && \
#     cp target/i686-linux-android/debug/libChemistryEquationsBackbone.so jniLibs/x86/libuniffi_chemistryequationsbackbone.so && \
#   mkdir -p jniLibs/x86_64/ && \
#     cp target/x86_64-linux-android/debug/libChemistryEquationsBackbone.so jniLibs/x86_64/libuniffi_chemistryequationsbackbone.so

# cargo run --features=uniffi/cli \
#     --bin uniffi-bindgen \
#     generate src/lib.udl \
#     --language kotlin
