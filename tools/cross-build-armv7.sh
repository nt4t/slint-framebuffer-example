#!/bin/bash
# Cross-compile slint-framebuffer-example for ARMv7 32-bit Linux
# Usage: ./cross-build-armv7.sh [output_dir]

set -e

OUTPUT_DIR="${1:-./target/armv7}"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== Cross-compile for ARMv7 ==="
echo "Project: $PROJECT_DIR"
echo "Output: $OUTPUT_DIR"

# Check prerequisites
echo ""
echo "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "ERROR: cargo not found. Install Rust first."
    exit 1
fi

if ! command -v arm-linux-gnueabihf-gcc &> /dev/null; then
    echo "ERROR: arm-linux-gnueabihf-gcc not found."
    echo "Install with: sudo apt-get install gcc-arm-linux-gnueabihf"
    exit 1
fi

# Add ARMv7 target if not already installed
echo ""
echo "Adding ARMv7 target..."
rustup target add thumbv7neon-unknown-linux-gnueabihf 2>/dev/null || true

# Ensure .cargo/config.toml exists
CARGO_CONFIG="$PROJECT_DIR/.cargo/config.toml"
if [ ! -f "$CARGO_CONFIG" ]; then
    echo ""
    echo "Creating .cargo/config.toml..."
    mkdir -p "$PROJECT_DIR/.cargo"
    cat > "$CARGO_CONFIG" << 'EOF'
[target.thumbv7neon-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
fi

# Clean previous build
echo ""
echo "Cleaning previous build..."
cd "$PROJECT_DIR"
rm -rf "target/thumbv7neon-unknown-linux-gnueabihf/release/slint-framebuffer-example"

# Build
echo ""
echo "Building for ARMv7..."
cargo build --target thumbv7neon-unknown-linux-gnueabihf --release

# Copy to output directory
echo ""
echo "Copying binary to $OUTPUT_DIR..."
mkdir -p "$OUTPUT_DIR"
cp "target/thumbv7neon-unknown-linux-gnueabihf/release/slint-framebuffer-example" "$OUTPUT_DIR/"

# Show binary info
echo ""
echo "=== Build Complete ==="
BINARY="$OUTPUT_DIR/slint-framebuffer-example"
file "$BINARY"
ls -lh "$BINARY"
echo ""
echo "To deploy:"
echo "  scp $BINARY user@arm-device:/path/to/"
echo ""
echo "Then on the ARM device:"
echo "  chmod +x slint-framebuffer-example"
echo "  ./slint-framebuffer-example"
