#!/usr/bin/env bash

set -euo pipefail

# --- Step 1: Start ChromeDriver ---
echo "🚀 Starting ChromeDriver on port 9515..."

if ! command -v chromedriver &> /dev/null; then
    echo "❌ chromedriver not found in PATH"
    exit 1
fi

chromedriver --port=9515 > chromedriver.log 2>&1 &
CHROMEDRIVER_PID=$!
echo "✅ ChromeDriver started (PID $CHROMEDRIVER_PID)"

# Optional: give it a moment to start
sleep 2

# --- Step 2: Build Rust project ---
echo "🔧 Building project in release mode..."
cargo build --release

# --- Step 3: Run compiled binary ---
BINARY_NAME=$(basename "$(pwd)")  # Assumes binary has same name as crate directory
BINARY_PATH="./target/release/$BINARY_NAME"

if [[ ! -f "$BINARY_PATH" ]]; then
    echo "❌ Binary not found at $BINARY_PATH"
    kill $CHROMEDRIVER_PID
    exit 1
fi

echo "🏃 Running $BINARY_PATH..."
"$BINARY_PATH"

# --- Step 4: Cleanup ChromeDriver after binary exits ---
echo "🛑 Stopping ChromeDriver (PID $CHROMEDRIVER_PID)..."
kill $CHROMEDRIVER_PID
