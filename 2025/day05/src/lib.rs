//! Day 5
//!
//! Solution scaffold for Advent of Code 2025 5.

use anyhow::{Context, Result, bail};

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

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
    let Some((intervals_str, numbers_str)) = input.split_once("\n\n") else {
        bail!("Input must contain both a set of fresh and availble ingredients")
    };
    let mut intervals = intervals_str
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once('-')
                .with_context(|| format!("missing '-' in line: {line:?}"))?;

            Ok((start.parse()?, end.parse()?))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;
    let mut numbers = numbers_str
        .lines()
        .map(|line| {
            line.parse()
                .with_context(|| format!("could not parse line to number: {line:?}"))
        })
        .collect::<Result<Vec<u64>>>()?;

    intervals.sort_unstable();
    numbers.sort_unstable();

    let mut i = 0;
    let mut j = 0;
    let mut total = 0;
    while i < intervals.len() && j < numbers.len() {
        let (start, end) = intervals[i];
        let number = numbers[j];
        if number > end {
            i += 1
        } else if number < start {
            j += 1
        } else {
            j += 1;
            total += 1
        }
    }
    Ok(total)
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
    let Some((intervals_str, _)) = input.split_once("\n\n") else {
        bail!("Input must contain both a set of fresh and available ingredients")
    };
    let mut intervals = intervals_str
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once('-')
                .with_context(|| format!("missing '-' in line: {line:?}"))?;

            Ok((start.parse()?, end.parse()?))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;

    intervals.sort_unstable();

    let mut total = 0;

    let Some((mut current_start, mut current_end)) = intervals.first().copied() else {
        return Ok(total);
    };

    for (next_start, next_end) in intervals.iter().skip(1).copied() {
        if next_start <= current_end + 1 {
            current_end = current_end.max(next_end)
        } else {
            total += current_end - current_start + 1;
            current_start = next_start;
            current_end = next_end
        }
    }

    total += current_end - current_start + 1;

    Ok(total)
}
