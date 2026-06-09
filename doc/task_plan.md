# Task Plan: Slint Framebuffer ARMv7 Project

## Goal
Cross-compile Slint framebuffer UI for ARMv7 32-bit Linux with keyboard navigation support.

## Target Platform
- Architecture: ARMv7 (armv7l, 32-bit ARM)
- OS: Linux (Debian/Ubuntu/ARM SBCs like Raspberry Pi)
- Framebuffer: /dev/fb0 (16-bit or 32-bit)
- Keyboard: /dev/input/event0
- Kernel: Linux 3.x+ with framebuffer support

## Phases

### Phase 1: Research & Setup (complete)
- [x] 1.1 Determine cross-compilation toolchain (arm-linux-gnueabihf-gcc)
- [x] 1.2 Install cross-compilation toolchain on build machine (Debian 13 x86_64)
- [x] 1.3 Configure Cargo for ARMv7 target (rustup target add thumbv7neon-unknown-linux-gnueabihf)
- [x] 1.4 Verify linuxfb crate compatibility with ARMv7 (pure Rust, no changes needed)

### Phase 2: Cross-Compilation Setup (complete)
- [x] 2.1 Download ARM libraries (fontconfig, freetype, expat, png, brotli, zlib)
- [x] 2.2 Create ARM sysroot with static libraries
- [x] 2.3 Configure .cargo/config.toml for cross-compilation
- [x] 2.4 Build with static font dependencies

### Phase 3: Slint Upgrade (complete)
- [x] 3.1 Upgrade Slint from 1.2 to 1.16
- [x] 3.2 Update slint-build to 1.16
- [x] 3.3 Fix API changes (renderer-software feature, callback methods)
- [x] 3.4 Update UI (dark background #1a1a1a)

### Phase 4: Keyboard Support (complete)
- [x] 4.1 Add evdev crate for keyboard input
- [x] 4.2 Read events from /dev/input/event0
- [x] 4.3 Map key codes (Up: 103/25/17, Down: 108/16/31, Enter: 28)
- [x] 4.4 Add debug logging for key events
- [x] 4.5 Fix UI updates (direct calls instead of invoke_from_event_loop)

### Phase 5: Framebuffer Improvements (complete)
- [x] 5.1 Support 32-bit framebuffers (RGB888/RGBA)
- [x] 5.2 Auto-detect framebuffer bit depth
- [x] 5.3 Convert RGB565 to RGB888 for 32-bit displays

### Phase 6: Documentation (complete)
- [x] 6.1 Create cross-compile guide (doc/cross-compile-guide.md)
- [x] 6.2 Update README with keyboard and framebuffer info
- [x] 6.3 Move planning docs to doc/ directory
- [x] 6.4 Document key codes and debugging

## Decisions

| Decision | Value | Reason |
|----------|-------|--------|
| Target triple | thumbv7neon-unknown-linux-gnueabihf | ARMv7 with NEON, hardware FP |
| C linker | arm-linux-gnueabihf-gcc | glibc cross-compiler available |
| Static linking | Font libs only (fontconfig, freetype, etc.) | glibc static linking problematic |
| Slint version | 1.16 | Latest stable, better API |
| Slint features | renderer-software | No X11/Wayland on framebuffer |
| Keyboard input | evdev crate | Pure Rust, cross-compiles fine |
| Keyboard device | /dev/input/event0 | Standard Linux input device |

## Errors Encountered
| Error | Resolution |
|-------|------------|
| rust-lld: error: symbols.o is incompatible with elf64-x86-64 | Added .cargo/config.toml with ARM linker |
| linking with arm-linux-gnueabihf-gcc failed: hidden symbol _start' isn't defined | Using dynamic linking for glibc |
| invoke_from_event_loop not working | Use direct UI method calls |
| Slint API changes (1.2 → 1.16) | Updated callback and invoke method names |

## Files Modified
- `.cargo/config.toml` — Cross-compilation config
- `Cargo.toml` — Added evdev, upgraded Slint to 1.16
- `src/main.rs` — Keyboard support, 32-bit FB, debug logging
- `ui/appwindow.slint` — Dark background (#1a1a1a)
- `README.md` — Keyboard and framebuffer documentation
- `doc/cross-compile-guide.md` — Cross-compile instructions
- `doc/findings.md` — Project findings
- `doc/progress.md` — Session progress
