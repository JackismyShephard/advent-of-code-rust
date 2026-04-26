//! Day 2
//!
//! Solution scaffold for Advent of Code 2025 2.

use anyhow::{bail, Context, Result};
use itertools::Itertools;

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

/// Solves Part 1 for the puzzle input.
///
/// # Parameters
/// * `input` - Raw puzzle input text
///
/// # Returns
/// The Part 1 answer once implemented
///
/// # Errors
/// Returns an error until the solution is implemented
pub fn solve_part1(input: &str) -> Result<u64> {
    let mut sum = 0;
    for range in input.split(",") {
        let (start, end) = range
            .split('-')
            .map(str::parse::<u64>)
            .collect_tuple()
            .with_context(|| format!("range must contain a start and end number: {range:?}"))
            .and_then(|(start, end)| Ok((start?, end?)))?;
        for n in start..=end {
            let num_digits = n.ilog10() + 1;
            let half_digits = num_digits / 2;
            let left_half = n / 10_u64.pow(half_digits);
            let right_half = n % 10_u64.pow(half_digits);
            if left_half == right_half {
                sum += n
            }
        }
    }
    Ok(sum)
}

/// Solves Part 2 for the puzzle input.
///
/// # Parameters
/// * `input` - Raw puzzle input text
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
