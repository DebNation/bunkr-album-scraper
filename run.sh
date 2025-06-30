#!/usr/bin/env bash


set -euo pipefail

# URL to download from
URL="https://github.com/uBlockOrigin/uBOL-home/releases/download/uBOLite_2025.624.1503/uBOLite_2025.624.1503.chromium.mv3.zip"

# Output paths
ZIP_NAME="uBOLite.zip"
DEST_DIR="./src/utils/adblock-extension"

# Create destination directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Download the ZIP file
echo "Downloading uBlock Origin Lite..."
curl -L "$URL" -o "$ZIP_NAME"

# Unzip to the target directory
echo "Unzipping to $DEST_DIR..."
unzip -o "$ZIP_NAME" -d "$DEST_DIR"

# Clean up
rm "$ZIP_NAME"
echo "Done."



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
