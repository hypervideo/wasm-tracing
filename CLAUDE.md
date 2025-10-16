# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`wasm-tracing` is a Rust library that bridges the `tracing` crate to browser performance and console APIs for WebAssembly applications. It's a maintained fork of the deprecated `tracing-wasm` crate.

**Core Architecture:**
- `WasmLayer`: Implements `tracing_subscriber::Layer` to intercept tracing events
- `StringRecorder`: Collects field data from spans/events for display
- `WasmLayerConfig`: Configuration for logging behavior, filtering, and console output
- `LogFilter`: Dynamic filtering wrapper around `EnvFilter` for runtime log directive updates

**Key Integration Points:**
- Uses `wasm-bindgen` to call browser `performance.mark()`, `performance.measure()`, and `console.log()`
- Works only in browser environments with `console` and `performance` globals (not Node.js/Workers)
- Hooks into tracing's subscriber system via `tracing-subscriber::Layer` trait

## Development Environment

This project uses Nix flakes for reproducible development environments.

**Setup:**
```bash
nix develop  # Enter development shell with Rust toolchain
```

The flake provides: rustc, cargo, clippy, rust-analyzer, rustfmt (nightly), and required system dependencies.

## Build and Test Commands

**Standard Cargo commands:**
```bash
cargo build              # Build the library
cargo test               # Run all tests (includes unit tests in src/)
cargo test <name>        # Run specific test
cargo clippy             # Lint the code
cargo fmt                # Format code
```

**WASM-specific testing:**
```bash
wasm-pack test --headless --firefox  # Run browser tests
wasm-pack test --headless --chrome   # Run browser tests in Chrome
```

Note: Tests in `tests/wasm.rs` use `wasm-bindgen-test` and must run in a browser environment.

## Library Structure

**Main entry points (src/lib.rs):**
- `set_as_global_default()` - Quick setup with defaults (panics on error)
- `try_set_as_global_default()` - Returns `Result` for error handling
- `set_as_global_default_with_config(config)` - Custom configuration
- `set_log_filter(directives)` - Runtime log level/filter updates (e.g., "info,my_crate=debug")

**Configuration (src/config/mod.rs):**
- `WasmLayerConfig` - Builder-pattern config with:
  - `report_logs_in_timings`: Show events in browser performance timeline
  - `console`: Console output mode (color/no-color/off)
  - `max_level`: Static max log level
  - `filter`: Dynamic `EnvFilter`-based filtering
  - `show_fields`: Display span/event fields
  - `show_origin`: Show file:line info
  - `origin_base_url`: URL prefix for clickable file paths

**Layer implementation (src/layer.rs):**
- Manages span lifecycle: records fields on creation, marks entry/exit for performance timing
- Filters events/spans via both static `max_level` and dynamic `LogFilter`
- Formats console output with optional color codes and field display

**Field recording (src/recorder.rs):**
- `StringRecorder` implements `tracing::field::Visit` to capture span/event fields as strings
- Stored in span extensions for access during exit (performance measurement)

## Important Notes

- The library uses a global `FILTER` static for runtime filter updates via `set_log_filter()`
- Thread indexing feature (`mark-with-rayon-thread-index`) appends thread info to performance marks
- Deprecated types: `WASMLayer`, `WASMLayerConfig` (use `Wasm*` variants)
- Recent addition: `origin_base_url` allows IDE integration via clickable console links
