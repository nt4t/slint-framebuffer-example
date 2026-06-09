# Cross-Compiling Slint Framebuffer for ARMv7 (32-bit)

## Prerequisites

### Build Host (x86_64 Linux)

```bash
# Install cross-compilation tools
sudo apt install gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf pkg-config

# Install Rust ARM target
rustup target add thumbv7neon-unknown-linux-gnueabihf
```

### Download ARM Libraries

Download the following Debian/Raspbian ARM packages from:
`http://raspbian.raspberrypi.com/raspbian/pool/main/`

**Font rendering stack packages:**
- fontconfig: `pool/main/f/fontconfig/`
- freetype: `pool/main/f/freetype/`
- libpng1.6: `pool/main/libp/libpng1.6/`
- brotli: `pool/main/b/brotli/`
- zlib: `pool/main/z/zlib/`
- expat: `pool/main/e/expat/`

**Required packages (Debian 12 / Raspbian bookworm):**

```
libfontconfig1_<version>_armhf.deb
libfontconfig-dev_<version>_armhf.deb
libfreetype6_<version>_armhf.deb
libfreetype-dev_<version>_armhf.deb
libpng16-16_<version>_armhf.deb
libpng-dev_<version>_armhf.deb
libbrotli1_<version>_armhf.deb
libbrotli-dev_<version>_armhf.deb
zlib1g_<version>_armhf.deb
zlib1g-dev_<version>_armhf.deb
libexpat1_<version>_armhf.deb
libexpat1-dev_<version>_armhf.deb
```

## Setup ARM Sysroot

```bash
# Create sysroot directory
SYSROOT=/tmp/arm-fontconfig
mkdir -p $SYSROOT/usr/lib/arm-linux-gnueabihf/pkgconfig

# Extract -dev packages (contain static libraries)
cd /tmp
mkdir arm-deps && cd arm-deps

# For each -dev package:
ar x /path/to/lib*-dev_*.deb
tar xf data.tar.xz
rm -f control.tar.xz data.tar.xz debian-binary

# Copy libraries to sysroot
cp -r usr/lib/arm-linux-gnueabihf/* $SYSROOT/usr/lib/arm-linux-gnueabihf/

# Remove shared libraries (force static linking)
rm -f $SYSROOT/usr/lib/arm-linux-gnueabihf/*.so
rm -f $SYSROOT/usr/lib/arm-linux-gnueabihf/*.so.*

# Copy .pc files
mkdir -p $SYSROOT/usr/lib/arm-linux-gnueabihf/pkgconfig
cp usr/lib/arm-linux-gnueabihf/pkgconfig/*.pc $SYSROOT/usr/lib/arm-linux-gnueabihf/pkgconfig/
```

## Configure Cargo

Create or edit `.cargo/config.toml`:

```toml
[target.thumbv7neon-unknown-linux-gnueabihf]
runner = "ssh user@<arm-device-ip>:"
linker = "arm-linux-gnueabihf-gcc"
```

## Build

```bash
# Set environment variables
export PKG_CONFIG_SYSROOT_DIR=/tmp/arm-fontconfig
export PKG_CONFIG_PATH=/tmp/arm-fontconfig/usr/lib/arm-linux-gnueabihf/pkgconfig
export PKG_CONFIG_ALLOW_CROSS=1

# Build
cargo build --release --target thumbv7neon-unknown-linux-gnueabihf \
    -C link-arg=-Wl,-Bstatic \
    -C link-arg=-lfontconfig \
    -C link-arg=-lfreetype \
    -C link-arg=-lexpat \
    -C link-arg=-lpng16 \
    -C link-arg=-lbrotlidec \
    -C link-arg=-lbrotlicommon \
    -C link-arg=-lz \
    -C link-arg=-Wl,-Bdynamic
```

## Deploy

```bash
# Copy binary to ARM device
scp target/thumbv7neon-unknown-linux-gnueabihf/release/<your-app> \
    user@<arm-device-ip>:~

# Run on device
ssh user@<arm-device-ip> "./<your-app>"
```

## Troubleshooting

### "undefined reference" errors
- Ensure all transitive dependencies are statically linked
- Check that `.a` files exist in the sysroot
- Verify pkg-config files point to correct paths

### "Exec format error" on device
- Verify binary is ARM: `file target/thumbv7neon-unknown-linux-gnueabihf/release/<app>`
- Should show: `ELF 32-bit LSB pie executable, ARM`

### pkg-config not finding libraries
- Set `PKG_CONFIG_SYSROOT_DIR` and `PKG_CONFIG_PATH`
- Set `PKG_CONFIG_ALLOW_CROSS=1`
- Verify `.pc` files have correct `prefix=/usr` (will be prepended by sysroot)
