# 🦀 Android Device Info (Rust)

Fast, minimal Rust CLI to grab Android device info via ADB.

## Build

```bash
cargo build --release
# Binary: target/release/adi
```

## Usage

```bash
adi              # pretty output
adi --json       # JSON output for scripting
adi --filter api # filter by keyword (case-insensitive)
```

Output includes: model, brand, Android version, API level, security patch, CPU, RAM, storage, battery.
