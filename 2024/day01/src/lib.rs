//! Day 1: Historian Hysteria
//!
//! Solution for Advent of Code 2024 Day 1.
//!
//! Part 1: Calculate total distance between two lists by pairing up
//! the smallest numbers and summing the absolute differences.
//!
//! Part 2: Calculate similarity score by multiplying each number in the left
//! list by how many times it appears in the right list, then summing.

use anyhow::{bail, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;

/// Example input from the problem statement used for testing and documentation.
pub const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

/// Solves Part 1: Calculates the total distance between the left and right
/// lists.
///
/// The function sorts both lists independently and then sums the absolute
/// differences of corresponding elements when paired by position.
///
/// # Parameters
/// * `input` - Multi-line string containing integer pairs
///   (whitespace-separated)
///
/// # Returns
/// Total distance as the sum of absolute differences between sorted pairs
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d01::solve_part1;
/// let input = "1 3\n2 5";
/// assert_eq!(solve_part1(input).unwrap(), 5);
/// // |1-3| + |2-5| = 2 + 3 = 5
/// ```
pub fn solve_part1(input: &str) -> Result<i32> {
    let (mut left_nums, mut right_nums) = parse_input(input)?;

    // Sort both lists
    left_nums.sort_unstable();
    right_nums.sort_unstable();

    // Calculate total distance using functional style
    let total_distance = left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(&left, &right)| (left - right).abs())
        .sum();

    Ok(total_distance)
}

/// Solves Part 2: Calculates a similarity score based on frequency matching.
///
/// For each unique number in the left list, multiplies the number by its
/// frequency in the left list and its frequency in the right list. Uses hash
/// maps for efficient frequency counting and handles duplicate values
/// optimally.
///
/// # Parameters
/// * `input` - Multi-line string containing integer pairs
///   (whitespace-separated)
///
/// # Returns
/// Similarity score as the sum of (left_number × left_frequency ×
/// right_frequency)
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d01::solve_part2;
/// let input = "3 3\n4 3\n2 3";
/// // 3 appears 3 times in right list: 3*3 = 9
/// // 4 appears 0 times in right list: 4*0 = 0
/// // 2 appears 0 times in right list: 2*0 = 0
/// assert_eq!(solve_part2(input).unwrap(), 9); // 9 + 0 + 0 = 9
/// ```
pub fn solve_part2(input: &str) -> Result<i32> {
    let (left_nums, right_nums) = parse_input(input)?;

    // Build frequency maps using FxHashMap for performance
    let right_counts = build_frequency_map(&right_nums);
    let left_counts = build_frequency_map(&left_nums);

    // Calculate similarity score using functional style
    let similarity_score = left_counts
        .iter()
        .map(|(&left_num, &left_freq)| {
            let right_freq = right_counts.get(&left_num).unwrap_or(&0);
            left_num * left_freq * right_freq
        })
        .sum();

    Ok(similarity_score)
}

/// Builds a frequency map using FxHashMap for optimal performance.
///
/// Creates a hash map counting how many times each number appears in the
/// input slice. Uses FxHashMap for better performance with integer keys
/// compared to standard HashMap.
///
/// # Parameters
/// * `nums` - Slice of integers to count frequencies for
///
/// # Returns
/// Hash map where keys are the unique numbers and values are their occurrence
/// counts
///
/// # Examples
///
/// ```
/// # use y2024_d01::build_frequency_map;
/// let freq_map = build_frequency_map(&[1, 2, 2, 3, 3, 3]);
/// assert_eq!(freq_map[&1], 1);
/// assert_eq!(freq_map[&2], 2);
/// assert_eq!(freq_map[&3], 3);
/// ```
pub fn build_frequency_map(nums: &[i32]) -> FxHashMap<i32, i32> {
    let mut counts = FxHashMap::default();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts
}
/// Naive O(n²) implementation of Part 2 for performance comparison.
///
/// Uses cartesian product (nested iteration) to compare every left number
/// with every right number, counting matches without hash map optimization.
///
/// # Parameters
/// * `input` - Multi-line string containing integer pairs
///   (whitespace-separated)
///
/// # Returns
/// Similarity score calculated using the naive O(n²) algorithm
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d01::solve_part2_naive;
/// let input = "3 3\n4 3\n2 3";
/// assert_eq!(solve_part2_naive(input).unwrap(), 9); // Same result as
///                                                    // optimized version
/// ```
pub fn solve_part2_naive(input: &str) -> Result<i32> {
    parse_input(input).map(|(left_nums, right_nums)| {
        left_nums
            .iter()
            .cartesian_product(right_nums.iter())
            .filter_map(|(&left, &right)| (left == right).then_some(left))
            .sum()
    })
}

/// Parses the input string into two separate lists of integers (left and
/// right columns).
///
/// Takes input with one pair of integers per line, separated by whitespace,
/// and separates them into left and right column vectors.
///
/// # Parameters
/// * `input` - Multi-line string with integer pairs (one pair per line,
///   whitespace-separated)
///
/// # Returns
/// Tuple of (left_column_numbers, right_column_numbers) as Vec<i32>
///
/// # Errors
///
/// Returns an error if:
/// - Any value cannot be parsed as an `i32`
/// - Any line doesn't contain exactly two whitespace-separated values
///
/// # Examples
///
/// ```
/// # use y2024_d01::parse_input;
/// let input = "1 2\n3 4";
/// let (left, right) = parse_input(input).unwrap();
/// assert_eq!(left, vec![1, 3]);
/// assert_eq!(right, vec![2, 4]);
/// ```
pub fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            [] => continue, // skip empty lines
            [left_str, right_str] => {
                left_nums.push(left_str.parse()?);
                right_nums.push(right_str.parse()?);
            }
            _ => bail!("Line must contain exactly two numbers: '{line}'"),
        }
    }

    Ok((left_nums, right_nums))
}
