# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a simple Rust learning project demonstrating basic Rust concepts like ownership, borrowing, and move semantics. The project uses Rust edition 2024.

## Common Commands

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

### Build (Release)
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Check Code (Fast Compile Check)
```bash
cargo check
```

### Format Code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

## Code Architecture

**Project Type**: Binary crate (single executable)

**Structure**:
- `src/main.rs`: Entry point containing all code
- No external dependencies currently defined
- Simple single-file architecture for learning purposes

**Current Implementation**:
The codebase demonstrates Rust ownership concepts with:
- Commented-out examples of basic functions, types, and control flow
- Active example showing String ownership and move semantics (note: current code has an intentional error for learning purposes)

## Development Notes

- This is a learning/experimental project
- Code contains commented examples for reference
- The active main() function demonstrates ownership concepts and may contain intentional errors for educational purposes
- No library dependencies are currently in use
