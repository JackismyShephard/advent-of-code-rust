//! Day 3
//!
//! Solution scaffold for Advent of Code 2025 3.

use anyhow::{Result, bail};

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str =
    "987654321111111\n811111111111119\n234234234234278\n818181911112111";

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
pub fn solve_part1(input: &str) -> Result<u32> {
    let mut total_sum = 0;
    for line in input.lines() {
        let mut first: u8 = 0;
        let mut second: u8 = 0;
        let bytes = line.as_bytes();
        let num_bytes = bytes.len();
        for (i, raw_byte) in bytes.iter().copied().enumerate() {
            if !raw_byte.is_ascii_digit() {
                bail!("Found non-digit ascii character in input")
            }
            let byte = raw_byte - b'0';
            if byte > first && i < num_bytes - 1 {
                first = byte;
                second = 0;
            } else if byte > second {
                second = byte;
            }
        }
        total_sum += (first * 10 + second) as u32;
    }
    Ok(total_sum)
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
pub fn solve_part2(input: &str, total_digits: usize) -> Result<u64> {
    let mut total_sum = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let num_bytes = bytes.len();
        if total_digits > num_bytes {
            bail!(
                "total number of desired digits must be less than or equal to number of ascii characters in input line"
            )
        }
        let mut stack = Vec::<u8>::with_capacity(total_digits);

        for (i, byte) in bytes.iter().copied().enumerate() {
            if !byte.is_ascii_digit() {
                bail!("Found non-digit ascii character in input")
            }
            let digit = byte - b'0';

            while num_bytes - i > total_digits - stack.len()
                && stack.last().is_some_and(|top| top < &digit)
            {
                stack.pop();
            }
            if stack.len() < total_digits {
                stack.push(digit);
            }
        }
        let mut number = 0_u64;
        for digit in stack.iter().copied() {
            number = number * 10 + digit as u64
        }
        total_sum += number;
    }
    Ok(total_sum)
}

pub fn solve_part2_old(input: &str, subseq_len: usize) -> Result<u64> {
    let mut total_sum = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let num_bytes = bytes.len();
        if subseq_len > num_bytes {
            bail!(
                "subsequence length must be less than or equal to number of ascii characters in input line"
            )
        }
        let mut subseq = vec![0; subseq_len];

        for (i, raw_byte) in bytes.iter().copied().enumerate() {
            if !raw_byte.is_ascii_digit() {
                bail!("Found non-digit ascii character in input")
            }
            let byte = raw_byte - b'0';
            let start = subseq_len.saturating_sub(num_bytes - i);
            for j in start..subseq_len {
                if byte > subseq[j] {
                    subseq[j] = byte;
                    subseq[j + 1..].fill(0);
                    break;
                }
            }
        }
        for (j, item) in subseq.iter().copied().enumerate() {
            total_sum += item as u64 * 10_u64.pow((subseq_len - j - 1) as u32)
        }
    }
    Ok(total_sum)
}
