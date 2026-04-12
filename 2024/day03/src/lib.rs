//! Day 3: Mull It Over
//!
//! Solution for Advent of Code 2024 Day 3.
//!
//! Part 1: Parse corrupted memory to find valid mul(X,Y) instructions
//! and sum their results. Valid instructions have the exact format
//! mul(X,Y) where X and Y are 1-3 digit numbers.
//!
//! Part 2: Handle conditional instructions do() and don't() that enable
//! and disable mul() instructions. Only mul() instructions after do()
//! (or at the start) are processed, while those after don't() are ignored.

use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

/// Example input for Part 2 with do() and don't() instructions.
pub const EXAMPLE_INPUT_PART2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

/// Solves Part 1: Sums the results of all valid multiplication instructions.
///
/// Scans corrupted memory for valid mul(X,Y) instructions, multiplies the
/// operands, and returns the sum of all multiplication results.
///
/// # Parameters
/// * `input` - String containing corrupted memory to parse
///
/// # Returns
/// Sum of all multiplication results
///
/// # Errors
///
/// Returns an error if instruction parsing fails (malformed numbers).
///
/// # Examples
///
/// ```
/// # use y2024_d03::solve_part1;
/// let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// assert_eq!(solve_part1(memory).unwrap(), 161); // 2*4 + 5*5 + 11*8 + 8*5 = 161
/// ```
pub fn solve_part1(input: &str) -> Result<u32> {
    extract_mul_instructions(input)
        .map(|instructions| instructions.iter().map(|(x, y)| x * y).sum())
}

/// Extracts all valid mul(X,Y) instructions from corrupted memory.
///
/// Uses regex pattern `mul\((\d{1,3}),(\d{1,3})\)` to find instructions with
/// the exact format mul(X,Y) where X and Y are 1-3 digit numbers. Invalid
/// formats like mul(4*, mul[3,7], or mul ( 2 , 4 ) are ignored.
///
/// # Parameters
/// * `input` - String containing corrupted memory with mixed valid/invalid
///   instructions
///
/// # Returns
/// Vector of (X, Y) tuples representing the operands of valid mul instructions
///
/// # Errors
///
/// Returns an error if any captured number cannot be parsed as a u32.
///
/// # Examples
///
/// ```
/// # use y2024_d03::extract_mul_instructions;
/// let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// let instructions = extract_mul_instructions(memory).unwrap();
/// assert_eq!(instructions, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
/// ```
pub fn extract_mul_instructions(input: &str) -> Result<Vec<(u32, u32)>> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .expect("Invalid regex pattern for mul instructions")
    });

    RE.captures_iter(input)
        .map(|captures| {
            let x = captures[1].parse()?;
            let y = captures[2].parse()?;
            Ok((x, y))
        })
        .collect()
}

/// Solves Part 2: Sums the results of enabled multiplication instructions.
///
/// Processes do() and don't() instructions to determine which mul instructions
/// are enabled, then multiplies the operands and returns the sum. mul
/// instructions are enabled by default at the start of the program.
///
/// # Parameters
/// * `input` - String containing corrupted memory to parse
///
/// # Returns
/// Sum of all enabled multiplication results
///
/// # Errors
///
/// Returns `Err` if instruction parsing fails (malformed numbers).
///
/// # Examples
///
/// ```
/// # use y2024_d03::solve_part2;
/// let memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
/// assert_eq!(solve_part2(memory).unwrap(), 48); // 2*4 + 8*5 = 48
/// ```
pub fn solve_part2(input: &str) -> Result<u32> {
    extract_enabled_mul_instructions(input)
        .map(|instructions| instructions.iter().map(|(x, y)| x * y).sum())
}
/// Extracts enabled mul(X,Y) instructions from corrupted memory.
///
/// Processes do() and don't() instructions to determine which mul instructions
/// are enabled. mul instructions are enabled by default at the start of the
/// program. The most recent do() or don't() instruction determines the current
/// state.
///
/// # Parameters
/// * `input` - String containing corrupted memory with mul, do(), and
///   don't() instructions
///
/// # Returns
/// Vector of (X, Y) tuples representing the operands of enabled mul
/// instructions
///
/// # Errors
///
/// Returns `Err` if any captured number cannot be parsed as a u32.
///
/// # Examples
///
/// ```
/// # use y2024_d03::extract_enabled_mul_instructions;
/// let memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
/// let instructions = extract_enabled_mul_instructions(memory).unwrap();
/// assert_eq!(instructions, vec![(2, 4), (8, 5)]);
/// ```
pub fn extract_enabled_mul_instructions(input: &str) -> Result<Vec<(u32, u32)>> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))")
            .expect("Invalid regex pattern for conditional mul instructions")
    });

    let mut enabled = true;
    let mut instructions = Vec::new();

    for captures in RE.captures_iter(input) {
        // captures[0] contains the entire match: "do()", "don't()", or "mul(X,Y)"
        // captures[1] and captures[2] contain the X and Y values for mul instructions
        let instruction = &captures[0];

        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                // This is a mul instruction
                if enabled {
                    let x = captures[1].parse()?;
                    let y = captures[2].parse()?;
                    instructions.push((x, y));
                }
            }
        }
    }

    Ok(instructions)
}
