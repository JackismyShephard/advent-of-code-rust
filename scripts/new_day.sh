#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage:
  scripts/new_day.sh YEAR DAY [--dry-run]

Examples:
  scripts/new_day.sh 2025 2
  scripts/new_day.sh 2025 02
  scripts/new_day.sh 2025 2 --dry-run

Creates a new Advent of Code day crate scaffold at YEAR/dayXX.
EOF
}

dry_run=false
positionals=()

for arg in "$@"; do
    case "$arg" in
        --dry-run)
            dry_run=true
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            positionals+=("$arg")
            ;;
    esac
done

if [[ "${#positionals[@]}" -ne 2 ]]; then
    usage >&2
    exit 1
fi

year="${positionals[0]}"
day_input="${positionals[1]}"

if [[ ! "$year" =~ ^20[0-9]{2}$ ]]; then
    echo "error: year must be a four-digit value like 2025" >&2
    exit 1
fi

if [[ ! "$day_input" =~ ^[0-9]{1,2}$ ]]; then
    echo "error: day must be a number from 1 to 25" >&2
    exit 1
fi

day_number=$((10#$day_input))

if (( day_number < 1 || day_number > 25 )); then
    echo "error: day must be between 1 and 25" >&2
    exit 1
fi

day_padded="$(printf '%02d' "$day_number")"
crate_dir="${year}/day${day_padded}"
package_name="y${year}-d${day_padded}"
lib_name="y${year}_d${day_padded}"
day_title="Day ${day_number}"

files=(
    "${crate_dir}/Cargo.toml"
    "${crate_dir}/description.txt"
    "${crate_dir}/input.txt"
    "${crate_dir}/src/lib.rs"
    "${crate_dir}/src/main.rs"
)

if [[ -e "$crate_dir" ]]; then
    echo "error: ${crate_dir} already exists" >&2
    exit 1
fi

if "$dry_run"; then
    echo "Would create:"
    for file in "${files[@]}"; do
        echo "  ${file}"
    done
    exit 0
fi

mkdir -p "${crate_dir}/src"

cat > "${crate_dir}/Cargo.toml" <<EOF
[package]
name = "${package_name}"
version.workspace = true
edition.workspace = true

[dependencies]
shared = { path = "../../shared" }
anyhow = { workspace = true }

[dev-dependencies]
rstest = { workspace = true }
EOF

cat > "${crate_dir}/description.txt" <<EOF
Advent of Code ${year} ${day_title}

Paste problem description excerpts here.
EOF

cat > "${crate_dir}/input.txt" <<'EOF'
EOF

cat > "${crate_dir}/src/lib.rs" <<EOF
//! ${day_title}
//!
//! Solution scaffold for Advent of Code ${year} ${day_number}.

use anyhow::{bail, Result};

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str = "";

/// Solves Part 1 for the puzzle input.
///
/// # Parameters
/// * \`input\` - Raw puzzle input text
///
/// # Returns
/// The Part 1 answer once implemented
///
/// # Errors
/// Returns an error until the solution is implemented
pub fn solve_part1(input: &str) -> Result<u64> {
    let _ = input;
    bail!("Part 1 not implemented yet")
}

/// Solves Part 2 for the puzzle input.
///
/// # Parameters
/// * \`input\` - Raw puzzle input text
///
/// # Returns
/// The Part 2 answer once implemented
///
/// # Errors
/// Returns an error until the solution is implemented
pub fn solve_part2(input: &str) -> Result<u64> {
    let _ = input;
    bail!("Part 2 not implemented yet")
}
EOF

cat > "${crate_dir}/src/main.rs" <<EOF
use anyhow::Result;
use ${lib_name}::{solve_part1, solve_part2, EXAMPLE_INPUT};

fn main() -> Result<()> {
    println!("=== ${day_title} ===");
    println!();

    println!("=== Example Input Results ===");
    match solve_part1(EXAMPLE_INPUT) {
        Ok(result) => println!("Part 1 example result: {result}"),
        Err(error) => println!("Part 1 example error: {error}"),
    }

    match solve_part2(EXAMPLE_INPUT) {
        Ok(result) => println!("Part 2 example result: {result}"),
        Err(error) => println!("Part 2 example error: {error}"),
    }
    println!();

    if let Ok(input) = shared::read_local_input!() {
        println!("=== Real Input Results ===");

        match solve_part1(&input) {
            Ok(result) => println!("Part 1 result: {result}"),
            Err(error) => println!("Part 1 error: {error}"),
        }

        match solve_part2(&input) {
            Ok(result) => println!("Part 2 result: {result}"),
            Err(error) => println!("Part 2 error: {error}"),
        }
    } else {
        println!("No input.txt found - create input.txt in this day crate with your puzzle input");
    }

    Ok(())
}
EOF

echo "Created ${crate_dir}"
echo "Package: ${package_name}"
echo "Next steps:"
echo "  1. Paste the puzzle text into ${crate_dir}/description.txt"
echo "  2. Paste your puzzle input into ${crate_dir}/input.txt"
echo "  3. Implement ${crate_dir}/src/lib.rs"
