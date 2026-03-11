# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`cc-statusline` is a Rust CLI tool that reads JSON configuration from stdin and makes HTTP requests to an external API (minimaxi.com) to check coding plan usage/remaining quota.

## Commands

```bash
# Build the project
cargo build

# Run the project (reads JSON from stdin)
cargo run -- < input.json

# Build for release
cargo build --release

# Run tests (if any)
cargo test

# Run a single test
cargo test <test_name>
```

## Architecture

- **Entry point**: `src/main.rs`
- **Input**: Reads JSON from stdin with the following structure:
  ```json
  {
    "model": { "id": "...", "display_name": "..." },
    "context_window": { "context_window_size": 123, "used_percentage": 50, "remaining_percentage": 50 }
  }
  ```
- **API Response structs** (in `src/main.rs`):
  - `ApiResponse`: Contains `base_resp` and `model_remains` array
  - `BaseResp`: `status_code` (i32), `status_msg` (String)
  - `ModelRemain`: `current_interval_total_count`, `current_interval_usage_count`, `end_time`, `model_name`, `remains_time`, `start_time`
- **Output**: Makes an authenticated GET request to minimaxi.com API and prints the parsed response
- **Dependencies**: serde, serde_json, tokio (async runtime), reqwest (HTTP client)

## Important Notes

- The Cargo.toml uses Rust edition 2024, which may require a nightly toolchain or be incorrect (current stable is 2021)
- There's a hardcoded Bearer token in the source code at line 43 - this should be moved to environment variables for security
