# Waiting Game (Pure Rust Version) 🦖

A ultra-minimal, high-performance Dino runner game built natively in Rust using the [Macroquad](https://macroquad.rs/) engine.

## 🚀 Performance
- **RAM Usage**: ~12MB (vs ~180MB for the Webview version)
- **CPU Usage**: 0% when idle
- **Binary Size**: ~2MB (stripped release)

## 🎮 How to Run
Ensure you have the Rust toolchain installed, then:
```bash
cargo run --release
```

## 🛠 Features
- Native Wayland/X11 rendering.
- Perfectly transparent-ready engine (requires compositor alpha support).
- Frame-perfect Dino physics.
- Optimized release profile (LTO, stripped symbols).

## ⚖️ License
MIT
