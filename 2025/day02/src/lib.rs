//! Day 2
//!
//! Solution scaffold for Advent of Code 2025 2.

use anyhow::{Context, Result};
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
        let (start, end) = parse_input(range)?;
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
pub fn solve_part2_old(input: &str) -> Result<u64> {
    let mut sum = 0;
    for range in input.split(",") {
        let (start, end) = parse_input(range)?;
        for n in start..=end {
            let num_digits = n.ilog10() + 1;
            for num_splits in get_unique_prime_divisors(num_digits) {
                let digits_per_split = num_digits / num_splits;
                let split_power = 10_u64.pow(digits_per_split);
                let segment = n % split_power;
                let reconstructed = segment * (split_power.pow(num_splits) - 1) / (split_power - 1);
                if reconstructed == n {
                    sum += n;
                    break;
                }
            }
        }
    }
    Ok(sum)
}

pub fn solve_part2(input: &str) -> Result<u64> {
    let mut sum = 0;
    for range in input.split(",") {
        let (start, end) = parse_input(range)?;
        let start_digits = start.ilog10() + 1;
        let end_digits = end.ilog10() + 1;
        for d in start_digits..=end_digits {
            let final_n = d / 2;
            let mut to_subtract = vec![0; final_n as usize + 1];
            for n in (1..=final_n).filter(|n| d % n == 0) {
                let r = d / n;
                let split_power = 10_u64.pow(n);
                let multiplier = (split_power.pow(r) - 1) / (split_power - 1);
                let min_segment_candidate = start.div_ceil(multiplier);
                let max_segment_candidate = end / multiplier;
                let min_segment_size = 10_u64.pow(n - 1);
                let max_segment_size = 10_u64.pow(n) - 1;
                let final_start_candidate = min_segment_candidate.max(min_segment_size);
                let final_end_candidate = max_segment_candidate.min(max_segment_size);
                if final_start_candidate > final_end_candidate {
                    continue;
                }
                let count_candidates = final_end_candidate - final_start_candidate + 1;
                let sum_n =
                    multiplier * count_candidates * (final_start_candidate + final_end_candidate)
                        / 2;
                let exact_sum_n = sum_n - to_subtract[n as usize];
                for j in (2 * n..=final_n).step_by(n as usize) {
                    to_subtract[j as usize] += exact_sum_n;
                }
                sum += exact_sum_n
            }
        }
    }
    Ok(sum)
}

fn parse_input(range: &str) -> Result<(u64, u64)> {
    let (start_str, end_str) = range
        .split('-')
        .collect_tuple()
        .with_context(|| format!("range must contain a start and end number: {range:?}"))?;
    let start = start_str.parse::<u64>()?;
    let end = end_str.parse::<u64>()?;
    Ok((start, end))
}

fn get_unique_prime_divisors(num: u32) -> Vec<u32> {
    let mut remaining = num;
    let mut prime_divisors = Vec::<u32>::with_capacity(num.ilog2() as usize);
    let trial_divisors = std::iter::once(2).chain((3..).step_by(2));
    for divisor in trial_divisors {
        if divisor > remaining / divisor {
            break;
        }
        if remaining % divisor == 0 {
            prime_divisors.push(divisor);
            while remaining % divisor == 0 {
                remaining /= divisor;
            }
        }
    }
    if remaining > 1 {
        prime_divisors.push(remaining)
    }
    prime_divisors
}
