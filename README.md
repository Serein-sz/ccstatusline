# ccstatusline

A customizable status line generator for Claude Code, written in Rust.

## Overview

ccstatusline reads JSON configuration from stdin and generates a colored status line with multiple segments displaying model info, directory, git status, context window usage, and API quota.

## Segments

| Segment | Description |
|---------|-------------|
| Model | Current Claude model (e.g., "MiniMax-M2.5") |
| Directory | Current working directory |
| Git | Branch name with status indicator (✓ clean, ● dirty, ⚠ conflicts) |
| Context Window | Usage percentage of context window |
| Extra | MiniMax API quota remaining (for MiniMax-M2.5 model) |

## Input Format

```json
{
  "model": {
    "id": "claude-opus-4-6",
    "display_name": "Claude Opus 4.6"
  },
  "workspace": {
    "current_dir": "/path/to/dir",
    "project_dir": "/path/to/project"
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 50,
    "remaining_percentage": 50
  }
}
```

## Usage

```bash
# Build
cargo build --release

# Run with JSON input
cargo run --release -- < input.json

# Run with debug output
cargo run --release -- --debug < input.json
```

## Dependencies

- [load_api_key](src/utils/loader.rs) reads API key from `~/.claude/settings.json` for MiniMax quota fetching

## License

MIT
