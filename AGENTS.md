# AGENTS.md

## Project Purpose

Learning Rust through Advent of Code challenges. Focus on hands-on coding, not
tutorials.

## Structure

- Rust workspace with separate packages for each day
- Year directories contain day crates, for example `2024/day01/`
- `shared/` package contains common utilities (input parsing, benchmarking,
  plotting, etc.)
- Run with: `cargo run -p y<year>-dXX`
- Test with: `cargo test -p y<year>-dXX`
- Benchmark with: `cargo bench -p y<year>-dXX` (add `--quiet` for minimal
  output)

## Next Steps for New Days

1. Create `<year>/dayXX/` directory
2. Copy an existing `<year>/dayXX/` crate and update the package name
3. Add `<year>/dayXX/description.txt` with problem description excerpts
4. Add `<year>/dayXX/input.txt` with puzzle input from AoC website
5. Create `<year>/dayXX/src/lib.rs` with core logic and `EXAMPLE_INPUT`
   constant
6. Create `<year>/dayXX/src/main.rs` with simple runner using lib functions
7. Create `<year>/dayXX/tests/dayXX.rs` with comprehensive tests (example +
   real input)
8. Test with example first, then run on real input
9. Optionally add `<year>/dayXX/benches/bench.rs` for performance benchmarks

## Setup Notes

- **Rust toolchain**:

  - Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Restart shell or run: `source ~/.cargo/env`

- Workspace members auto-detected by `members = ["20*/day*", "shared"]`
  pattern

### Rust-Analyzer Settings (Beginner-Optimized)

Focused configuration for learning Rust without information overload:

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

**Future Additions (After Month 1):**

- Type hints, reborrow hints, expression adjustments (when comfortable with
  ownership)
- Advanced inlay hints (closures, discriminants, binding modes)

## Development Workflow (For Codex)

- **When user says "remember" something: ALWAYS add it to this AGENTS.md file**

  - User instructions prefixed with "remember" should be documented here
  - This creates a persistent record of important project-specific guidance

- **ALWAYS run commands from the project root directory**

  - Never run commands from subdirectories like `2024/day01/` or `shared/`
  - This prevents path confusion and ensures consistent behavior
  - Use `cargo bench -p y<year>-dXX` format for package-specific operations

- **after completing a task ALWAYS**

  - stage relevant files with `git add`
  - run pre-commit hooks to ensure code quality
  - DO NOT commit unless asked to do so.

- **IMPORTANT: When commiting always include all changes for the given task:**

  - `<year>/dayXX/src/main.rs` - The solution code
  - `<year>/dayXX/Cargo.toml` - Dependencies
  - `<year>/dayXX/input.txt` - Personal puzzle input
  - `<year>/dayXX/description.txt` - Problem description excerpts
  - `AGENTS.md` - Update implementation status or persistent guidance
  - `README.md` - Update current status section
  - `Cargo.lock` - Dependency lock

### Git conventions

- **ALWAYS follow these git commit message guidelines**

- Use concise, one-line commit messages
- Example: "Initial commit: Advent of Code Rust workspace setup"
- **NEVER include co-authoring or Codex references in commit messages**

## Code Quality Enforcement

### Pre-commit Hooks (Active)

**Purpose**: Automatically enforce code quality standards before each commit,
preventing broken or poorly formatted code from entering the repository.

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
- **Markdown files**: Will be added in future updates

**IMPORTANT - Never bypass hooks:**

- Never use `git commit --no-verify` unless absolutely necessary
- If hooks fail, fix the issues rather than bypassing them
- This maintains code quality and prevents technical debt

### Future Consideration: Migration to Just Command Runner

If pre-commit hooks become too intrusive or if we want more manual control
over quality checks, consider migrating to a `justfile`-based approach.
Replace automatic pre-commit hooks with manual `just check` commands.
Would require discipline to remember running checks before commits

### Common Linting/Type Errors and Fixes

**Remember these fixes for recurring issues:**

- **`clippy::uninlined_format_args`**: Use `{variable}` instead of `{}",
  variable` in format strings
  - ❌ `println!("Result: {}", result);`
  - ✅ `println!("Result: {result}");`

- **`clippy::too_many_arguments`**: NEVER use
  `#[allow(clippy::too_many_arguments)]` without explicit user permission
  - ✅ **Proper solution**: Group semantically related parameters into
    meaningful structs
  - ❌ **Wrong**: Adding `#[allow]` annotation to bypass the warning

- **Docstring line breaking**: NEVER use `\` continuation in comments
  - ❌ Breaking mid-word or mid-phrase creates unreadable text like
    "/// input\n/// slice" or "/// instructions\n/// are enabled"
  - ✅ Break at natural sentence/phrase boundaries for readability
  - ✅ Continue on next line with proper `///` prefix and proper spacing
  - ✅ Extend continuation lines to ~80 characters when possible unless at
    section end

## Workspace Dependency Management

- **Shared crates go in workspace dependencies**: If multiple days use the same
  crate, add to `[workspace.dependencies]`
- **Check for duplication**: Before adding dependencies, see if other days
  already use them
- **Keep workspace.dependencies updated**: Maintain central dependency
  management

## Rust Coding Conventions

- **Avoid over-engineering**: Don't use structs/classes unless explicitly
  requested or benefit is TOTALLY TOTALLY obvious

### Rust Signed / unsigned Problem

**The Core Tension:**

- **Array indexing**: Requires `usize` (unsigned)
  - language requirement to prevent negative array access
- **Arithmetic**: Often needs signed values.
- casting between signed and unsigned types leads to verbose code

### SOlution: All-Signed Internal (Professional Grid Libraries)

```rust
struct Coord { x: i32, y: i32 }  // grid_2d crate approach
```

- **Pros**: Simple math, proven in production, industry standard
- **Cons**: Conversion overhead at indexing points

## Error Handling Rules

- **NEVER use `assert_eq!` in library code**: `assert_eq!` is only for tests,
  not runtime validation
- **NEVER use `unwrap()` in library code**: `unwrap()` is primarily for tests,
  causes panics in production
- **Library functions should return `Result<T, E>`**: Allow callers to handle
  errors gracefully
- **Use `?` operator for error propagation**: `value.parse::<u32>()?` instead
  of `value.parse::<u32>().unwrap()`
- **Use `bail!` for simple error returns**: More concise than
  `return Err(anyhow!(...))` - `bail!` macro handles the return automatically
- **Use let-else pattern matching for validation**:

- **Prefer pattern matching over indexing**: Safer and more idiomatic than
  bounds checking + indexing
- **NEVER overwrite description.txt**: Always preserve original AoC problem
  text, add analysis to separate files

## Documentation Best Practices

### Rust Documentation Standards

Following Rust ecosystem conventions for writing high-quality documentation:

#### Standard Documentation Sections (in order)

1. **Brief description** - What the function does (first paragraph)
2. **Detailed explanation** - How it works, algorithms used, performance
   characteristics
3. **`# Parameters`** - **Explicit semantic documentation of what each
   parameter represents**
4. **`# Returns`** - **Explicit description of return value meaning and units**
   - ONLY for functions that return meaningful values, never for functions
     returning `()`
5. **`# Errors`** - When function returns `Result<T, E>` (fallible functions)
6. **`# Panics`** - When function can panic (if applicable)
7. **`# Examples`** - **SINGLE SIMPLE EXAMPLE for demonstration only**
   - Use `# use` to import necessary items
   - Keep minimal - just show basic usage
8. **`# Safety`** - For unsafe functions only

#### Project-Specific Enhancement: Explicit Semantic Documentation

**Philosophy**: While Rust's type system is excellent, it doesn't always convey
the *semantic meaning* of parameters and return values. We enhance standard Rust
docs with explicit parameter and return documentation.

#### What NOT to include

- Redundant type information - Function signature is self-documenting for types
- Overly verbose descriptions - Keep semantic descriptions concise but complete
- **Multiple assertions in doctests** - Prefer single assert per doctest when
  possible

#### Line Length Standard

- **Docstrings/Comments**: 80 characters maximum (official Rust Style Guide
  standard)
- **Enforcement**: Automated via pre-commit hook + manual code review
- **Rationale**: Follows RFC 2436 specification: "Source lines which are
  entirely comments should be limited to 80 characters"

### Generating Documentation

```bash
# Generate and open documentation in browser
cargo doc --no-deps --open -p y<year>-dXX

# Generate documentation for all packages
cargo doc --no-deps
```

#### WSL-Specific Documentation Viewing

If `cargo doc --open` fails with permission errors in WSL:

```bash
# Fix permissions
sudo chown -R $USER:$USER target/

# Install wslu and use wslview
sudo apt install wslu
cargo doc --no-deps
wslview target/doc/y<year>_dXX/index.html
```

## Writing tests

**Tests go in tests/dayXX.rs**: Not in src/lib.rs with `#[cfg(test)]`, despite
that being standard Rust convention

**Consistent organization across all days:**

```rust
// ===== PARSE INPUT TESTS =====
// Basic functionality and edge cases

// ===== CORE FUNCTION TESTS =====
// Parameterized tests for individual functions

// ===== SOLVE FUNCTION TESTS =====
// Example input, real input, and edge cases
```

**Best Practice:** Inline comments explaining each test case

- **ALWAYS test exact error messages when error messages are added to code**
- Use `result.unwrap_err().to_string()` and `assert!()` with `contains()` for
  partial matches
- Or `assert_eq!()` for exact matches when error message is completely
  controlled

- **Nested Parameterization for Algorithm Testing**

  - **Use `#[values]` to test multiple algorithm implementations with same test
    cases**
  - Reduces code duplication when testing naive vs optimized implementations
  - Ensures both algorithms are tested identically against all edge cases
  - Pattern for testing multiple functions:

    ```rust
    #[rstest]
    #[case(test_data1, expected1)]  // test case 1
    #[case(test_data2, expected2)]  // test case 2
    fn test_algorithms(
        #[values(algorithm1, algorithm2)] algo: fn(Input) -> Output,
        #[case] input: Input,
        #[case] expected: Output,
    ) {
        assert_eq!(algo(input), expected, "Failed for input: {input:?}");
    }
    ```

  - **Organise tests by functionality, not by algorithm** - group related test
    cases together
  - **Use separate test functions for different categories** (basic, edge
    cases, complex scenarios)
  - **Comprehensive edge case testing through parameterization:**

- **Test Parameterization Style - Prefer Comments Over Description Parameters**

  - Use inline comments `// description` instead of description parameters in
    `#[case]`
  - Use `{input:?}` or `{sequence:?}` in error messages for debugging

  - Example pattern:

    ```rust
    #[rstest]
    #[case("input1", expected1)]  // test case 1 description
    #[case("input2", expected2)]  // test case 2 description
    fn test_function(#[case] input: &str, #[case] expected: Type) {
        assert_eq!(result, expected, "Failed for input: {input:?}");
    }
    ```

## Benchmark Structure

- **Follow day01 benchmark pattern**: Use descriptive names like
  `algorithm_vs_baseline.rs`
- **Test multiple input sizes**: Show algorithmic complexity scaling behavior
- **Generate comparative graphs**: Criterion should output comparison
  visualizations
- **Use modern criterion APIs**: Avoid deprecated functions like
  `criterion::black_box`

### IMPORTANT: Benchmark Configuration Requirements

- **Rust benchmarks require explicit Cargo.toml configuration** - they are NOT
  auto-discovered like tests
- **ALWAYS add `[[bench]]` entries to Cargo.toml**:

  ```toml
  [[bench]]
  name = "your_benchmark_name"
  harness = false
  ```

- **Without proper Cargo.toml entry, benchmarks will hang after compilation
  with no output**
- **Each benchmark file needs its own `[[bench]]` entry**

### Criterion HTML Reports

Criterion automatically generates HTML reports when benchmarks are run. The
reports are located at:

- `<year>/dayXX/data/criterion/report/index.html` (main report index)
- Individual benchmark reports in subdirectories

**For full HTML report functionality, install gnuplot:**

```bash
# Ubuntu/Debian
sudo apt install gnuplot

# macOS
brew install gnuplot

# Windows
# Download from http://www.gnuplot.info/download.html
```

### Report Features

- **Performance timelines**: Track benchmark changes over time
- **Detailed statistics**: Confidence intervals, outlier detection
- **Comparative analysis**: Side-by-side performance comparisons
- **Interactive plots**: Zoom, pan, and explore data points

## Research Methodology

**When user requests "ultrathink and research" or "deep research":**

### Phase 1: Primary Source Investigation

- **RFCs & Language Issues**: Search rust-lang/rfcs for design decisions and
  rationale
- **Library Documentation**: Read actual API docs, not just descriptions
- **Community Forums**: Access Rust Internals, not just user forums
- **Version History**: Research when features were added and why

### Phase 2: Cross-Domain Analysis

- **Multiple Domains**: Check game engines, scientific computing, image
  processing, etc..
- **Real Codebases**: Examine how major libraries solve problems
- **Performance Data**: Look for benchmarks and real-world measurements
- **Historical Context**: Understand evolution of approaches over time

### Phase 3: Evidence Quality Assessment

- **Authoritative**: Language RFCs, core team discussions, library maintainer
  posts
- **Practical**: Stack Overflow with multiple upvotes, real project examples
- **Speculative**: Blog posts, opinions without backing data
- **Verify Access**: Confirm you can actually read sources, not just
  descriptions

### Phase 4: Honest Reporting

- **Qualify Claims**: "Based on limited sources" vs "definitive consensus"
- **Cite Limitations**: Note 403 errors, paywalls, incomplete access
- **Multiple Perspectives**: Present competing viewpoints with evidence
- **Avoid Extrapolation**: Don't claim "industry standard" from one example

## Performance Optimization & Profiling

**IMPORTANT**: Always use `--release` flag for performance-sensitive code

### Profiling Workflow

#### Step 1: Establish Baseline with Criterion

#### Step 2: Component-Level Analysis

- Create separate benchmarks for each major component to identify bottlenecks:

- Example: Break down parsing vs sorting vs calculation

#### Step 3: Profile-Guided Optimization

Target the **biggest bottleneck first**

#### Step 4: Verify Improvements

### Optimization Priority (Proven Effective)

1. **Algorithm choice** - O(n) vs O(n²) (day01: 7.5x speedup HashMap vs naive)
2. **Data structure optimization** - HashMap vs Vec, pre-allocation
3. **Parsing optimization** - SIMD libraries for known formats (day01: 1.67x
   speedup)
4. **Compiler optimizations** - Always use `--release` mode
5. **Profile-Guided Optimization (PGO)**
6. **Manual micro-optimizations**
7. **Parallelization**
