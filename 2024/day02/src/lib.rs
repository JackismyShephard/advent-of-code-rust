//! Day 2: Red-Nosed Reports
//!
//! Solution for Advent of Code 2024 Day 2.
//!
//! Part 1: Analyze reactor safety reports to determine which are safe.
//! A report is safe if levels are all increasing or all decreasing,
//! and adjacent levels differ by 1-3.
//!
//! Part 2: Problem Dampener - allows removing one level to make unsafe
//! reports safe. If removing any single level makes a report safe,
//! then the report is considered safe.

use anyhow::Result;
use itertools::Itertools;

/// Example input from the problem statement used for testing and documentation.
pub const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

/// Minimum safe difference between adjacent levels in a reactor report.
const MIN_SAFE_DIFF: i32 = 1;

/// Maximum safe difference between adjacent levels in a reactor report.
const MAX_SAFE_DIFF: i32 = 3;

/// Solves Part 1: Counts how many reports are safe.
///
/// Analyzes each reactor report to determine if it meets safety criteria:
/// all levels increasing/decreasing with adjacent differences of 1-3.
///
/// # Parameters
/// * `input` - Multi-line string containing reactor level reports
///
/// # Returns
/// Number of safe reports as an integer
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d02::solve_part1;
/// let input = "7 6 4 2 1\n1 3 6 7 9";
/// assert_eq!(solve_part1(input).unwrap(), 2); // Both reports are safe
/// ```
pub fn solve_part1(input: &str) -> Result<usize> {
    parse_input(input).map(|reports| reports.iter().filter(|report| is_safe(report)).count())
}

/// Checks if a report is safe according to reactor safety rules.
///
/// A report is safe if:
/// 1. All levels are either increasing or decreasing
/// 2. Adjacent levels differ by at least 1 and at most 3
///
/// # Parameters
/// * `report` - Vector of reactor levels to analyze for safety
///
/// # Returns
/// True if the report meets all safety criteria, false otherwise
///
/// # Examples
///
/// ```
/// # use y2024_d02::is_safe;
/// assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true); // Decreasing by 1-2
/// assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false); // Jump of 5
/// assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false); // No change (4->4)
/// ```
pub fn is_safe(report: &[i32]) -> bool {
    let mut direction = None;

    report.iter().tuple_windows().all(|(a, b)| {
        let diff = b - a;

        // Check if difference is within valid range
        if diff.abs() < MIN_SAFE_DIFF || diff.abs() > MAX_SAFE_DIFF {
            return false;
        }

        // Check/establish monotonicity
        let is_increasing = diff > 0;
        match direction {
            None => {
                direction = Some(is_increasing);
                true
            }
            Some(dir) => dir == is_increasing,
        }
    })
}

/// Solves Part 2: Counts how many reports are safe with the Problem
/// Dampener.
///
/// Analyzes each reactor report to determine if it meets safety criteria
/// either directly or after removing exactly one level. The Problem Dampener
/// allows the reactor safety systems to tolerate a single bad level.
///
/// # Parameters
/// * `input` - Multi-line string containing reactor level reports
///
/// # Returns
/// Number of safe reports (including those made safe by dampening) as an
/// integer
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d02::solve_part2;
/// let input = "7 6 4 2 1\n1 3 2 4 5\n8 6 4 4 1";
/// assert_eq!(solve_part2(input).unwrap(), 3); // All can be made safe
/// ```
pub fn solve_part2(input: &str) -> Result<usize> {
    parse_input(input).map(|reports| {
        reports
            .iter()
            .filter(|report| is_safe_with_dampener(report))
            .count()
    })
}

/// Checks if a report is safe with the Problem Dampener active.
///
/// The Problem Dampener allows removing exactly one level from an unsafe
/// report to make it safe. A report is considered safe if it's either
/// already safe, or becomes safe after removing any single level.
///
/// # Parameters
/// * `report` - Vector of reactor levels to analyze with dampening capability
///
/// # Returns
/// True if the report is safe or can be made safe by removing one level
///
/// # Examples
///
/// ```
/// # use y2024_d02::is_safe_with_dampener;
/// assert!(is_safe_with_dampener(&[7, 6, 4, 2, 1])); // Already safe
/// assert!(is_safe_with_dampener(&[1, 3, 2, 4, 5])); // Safe by removing 3
/// assert!(is_safe_with_dampener(&[8, 6, 4, 4, 1])); // Safe by removing
///                                                   // one 4
/// assert!(!is_safe_with_dampener(&[1, 2, 7, 8, 9])); // Cannot be made
///                                                    // safe - jumps too
///                                                    // large
/// ```
pub fn is_safe_with_dampener(report: &[i32]) -> bool {
    // Check if already safe without removing any elements
    is_safe(report)
        // Or try removing each element one by one until we find a safe sequence
        || (0..report.len()).any(|i| {
            // Create new sequence without element at index i
            let sequence: Vec<i32> = report[..i]// Elements before index i
                .iter()
                .chain(report[i + 1..].iter()) // Elements after index i
                .copied()
                .collect();
            // Check if this dampened sequence is safe
            is_safe(&sequence)
        })
}

/// Parses the input string into a vector of reports, where each report is a
/// vector of levels.
///
/// Each line contains space-separated integers representing reactor levels.
///
/// # Parameters
/// * `input` - Multi-line string with reactor level reports (one report per
///   line, space-separated integers)
///
/// # Returns
/// Vector of reports, where each report is a Vec<i32> of levels
///
/// # Errors
///
/// Returns an error if any value cannot be parsed as an `i32`.
///
/// # Examples
///
/// ```
/// # use y2024_d02::parse_input;
/// let input = "1 2 3\n4 5 6";
/// let reports = parse_input(input).unwrap();
/// assert_eq!(reports, vec![vec![1, 2, 3], vec![4, 5, 6]]);
/// ```
pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let levels = line.split_whitespace().map(|s| s.parse()).try_collect()?;
            Ok(levels)
        })
        .collect()
}
