# Advent of Code - Rust Workspace

Learning Rust through Advent of Code challenges.

This repository is intended to stay reusable across Advent of Code years.
Each year lives in its own directory, and day crates use short year-scoped
package names to avoid workspace collisions.

## Structure

- `2024/day01/`, `2024/day02/`, etc. - Individual day solutions grouped by year
- `shared/` - Common utilities for input parsing
- Each day is a separate Rust package in the workspace

## Running Solutions

```bash
# Run solution
cargo run -p y<year>-dXX

# Run tests
cargo test -p y<year>-dXX

# Run benchmarks
cargo bench -p y<year>-dXX
```

## Current Status

- ✅ Day 1: Complete
  - Part 1: Distance calculation between sorted lists
    (result: 11 for example, 1,603,498 for real input)
  - Part 2: Similarity score using frequency maps
    (result: 31 for example, 25,574,739 for real input)
  - Performance benchmark: O(n) vs O(n²) comparison with visual graph generation

- ✅ Day 2: Complete
  - Part 1: Reactor safety report analysis
    (result: 2 for example, 686 for real input)
  - Part 2: Problem Dampener - allows removing one level to make unsafe reports safe
    (result: 4 for example, 717 for real input)
  
- ✅ Day 3: Complete
  - Part 1: Corrupted memory mul instruction parsing
    (result: 161 for example, 190,604,937 for real input)
  - Part 2: do()/don't() conditional processing
    (result: 48 for example, 82,857,512 for real input)

- 🔄 Day 4: Part 1 Complete
  - Part 1: XMAS word search in 2D grid (8 directions)
    (result: 18 for example, TBD for real input)
  - Part 2: TBD

## Adding New Days

1. Create `<year>/dayXX/`
2. Add problem description as `<year>/dayXX/description.txt`
3. Add input file as `<year>/dayXX/input.txt`
4. Copy an existing `<year>/dayXX/Cargo.toml` and update the package name to `y<year>-dXX`
5. Create `<year>/dayXX/src/lib.rs` with core logic and `EXAMPLE_INPUT`
6. Create `<year>/dayXX/src/main.rs` with a simple runner using lib functions
7. Create `<year>/dayXX/tests/dayXX.rs` with comprehensive tests
8. Optionally add `<year>/dayXX/benches/bench.rs` for performance benchmarks

## Performance Analysis

Some days include performance benchmarks comparing different algorithmic approaches:

- **Day 1**: Run `cargo bench -p y2024-d01` to generate `performance_comparison.svg`
- Shows performance scaling between optimized hashmap approach vs naive nested loops
- Demonstrates clear O(n) vs O(n²) performance differences with speedup factors
- **Day 2**: Run `cargo bench -p y2024-d02` for micro-benchmarks
- Compares different safety checking approaches and Problem Dampener implementations

## Setup Notes

- **Rust toolchain**:
  - Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Restart shell or run: `source ~/.cargo/env`

### Rust-Analyzer Settings (Beginner-Optimized)

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.diagnostics.styleLints.enable": true,
  "rust-analyzer.completion.fullFunctionSignatures.enable": true,
  "rust-analyzer.testExplorer": true,
  "rust-analyzer.inlayHints.lifetimeElisionHints.enable": "skip_trivial",
  "rust-analyzer.imports.granularity.enforce": true,
  "rust-analyzer.restartServerOnConfigChange": true
}
```

## Code Quality Enforcement

**Setup Instructions:**

```bash
# Download latest pre-commit (avoids nodeenv issues with apt version)
curl -LO https://github.com/pre-commit/pre-commit/releases/download/v4.2.0/pre-commit-4.2.0.pyz

# Install hooks in repository (one-time setup)
python3 pre-commit-4.2.0.pyz install

# Test hooks manually (optional)
python3 pre-commit-4.2.0.pyz run --all-files
```

**What the hooks check:**

- **Rust code**: Formatting (rustfmt) and linting (clippy with strict warnings)
- **Tests**: All unit and integration tests must pass (cargo test)
- **TOML files**: Formatting and linting (taplo)
- **Markdown files**: Formatting and linting (markdownlint)

## Documentation

### Generating Documentation

```bash

# Generate and open documentation in browser
cargo doc --no-deps --open -p y<year>-dXX

# Generate documentation for all packages
cargo doc --no-deps
```

### WSL-Specific Documentation Viewing

If `cargo doc --open` fails with permission errors in WSL:

```bash
# Fix permissions
sudo chown -R $USER:$USER target/

# Install wslu and use wslview
sudo apt install wslu
cargo doc --no-deps
wslview target/doc/y<year>_dXX/index.html
```
