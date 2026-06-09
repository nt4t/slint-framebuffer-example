# Findings: Slint Framebuffer ARMv7 Project

## Project State
- **Platform**: Debian 13 x86_64 (build machine)
- **Target**: ARMv7 32-bit Linux (Debian/ARM SBC)
- **Framebuffer**: Supports both 16-bit (RGB565) and 32-bit (RGB888/RGBA) framebuffers
- **Slint**: 1.16 with renderer-software feature
- **Keyboard**: /dev/input/event0 via evdev crate

## Cross-Compilation Target Triple
- **thumbv7neon-unknown-linux-gnueabihf**: ARMv7 with NEON SIMD, hard-float
  - `thumbv7` = ARMv7 with Thumb-2 instruction set
  - `neon` = ARM NEON SIMD support (good for pixel operations)
  - `gnueabihf` = GNU with hard-float ABI

## Dependencies

### linuxfb (git dependency)
- Pure Rust crate, no C dependencies
- Uses `libc` crate (cross-compiles fine)
- Works on ARMv7 without changes

### ctrlc (3.4.1)
- Supports ARM Linux via `nix` crate
- Cross-compiles fine

### Slint (1.16)
- `renderer-software` feature is pure Rust
- Cross-compiles for ARMv7 without issues

### evdev (0.12.0)
- Provides keyboard input from /dev/input/event0
- Pure Rust, cross-compiles fine

## Keyboard Input
- Reads from `/dev/input/event0`
- Uses evdev crate for raw input events
- Key codes vary by keyboard layout:
  - Up: 103 (Arrow Up), 25 (W), 17
  - Down: 108 (Arrow Down), 16 (S), 31
  - Select: 28 (Enter)

## Framebuffer Support
- Auto-detects 16-bit and 32-bit framebuffers
- 16-bit: Direct RGB565 write
- 32-bit: Converts RGB565 to RGB888

## Build Result
- **Binary**: `target/thumbv7neon-unknown-linux-gnueabihf/release/slint-framebuffer-example`
- **Size**: 6.5MB
- **Architecture**: ELF 32-bit LSB pie executable, ARM, EABI5
- **Linking**: Dynamically linked (libgcc_s.so.1, libm.so.6, libc.so.6, ld-linux-armhf.so.3)
- **Compatibility**: Works on ARMv7 systems with glibc (Debian/Ubuntu ARM)
