#!/bin/bash
# copy-backend.sh (Bash – Unix or Git Bash on Windows)

echo "Building backend..."
cargo build --release --manifest-path backend/Cargo.toml

BIN_NAME="mggraph-backend"
TARGET_DIR="backend/target/release"
DEST_DIR="src-tauri/bin"

# On Windows, binary ends in .exe
if [[ "$OS" == "Windows_NT" ]]; then
  BIN_FILE="$BIN_NAME.exe"
else
  BIN_FILE="$BIN_NAME"
fi

mkdir -p "$DEST_DIR"
cp "$TARGET_DIR/$BIN_FILE" "$DEST_DIR/$BIN_FILE"