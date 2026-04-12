//! Day 5: Print Queue
//!
//! Solution for Advent of Code 2024 Day 5.
//!
//! Part 1: Validates page sequences against precedence rules to determine which
//! updates are in the correct order and sum their middle page numbers.
//!
//! ## Problem Overview
//!
//! This problem is fundamentally about **topological ordering validation**.
//! Given a set of precedence constraints (X must come before Y) and sequences
//! of elements, we need to determine which sequences respect all applicable
//! constraints.
//!
//! ## Mathematical Framework
//!
//! The problem can be modeled as a directed graph where:
//! - Each page number is a vertex
//! - Each rule X|Y creates a directed edge from X to Y
//! - A valid sequence represents a topological ordering of the subgraph
//!   containing only the pages in that sequence
//!
//! ## Algorithmic Approaches
//!
//! 1. **Naive O(N²M)**: For each pair of pages in sequence, check all rules
//! 2. **Optimized O(N + M)**: Build position map, then validate each rule in
//!    constant time
//!
//! The optimized approach leverages the insight that we only need to check rule
//! violations, not construct a full topological sort.
//!
//! **Note on Graph Theory Notation**: In formal graph algorithms, this would be
//! expressed as O(V + E) where V = vertices (unique pages) and E = edges (rules).
//! In our specific context: V ≤ N (pages in sequence) and E = M (total rules),
//! so O(V + E) ≈ O(N + M) for practical analysis.

use anyhow::{Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;

/// Type alias for ordering rules: list of (before_page, after_page) pairs
type Rules = Vec<(u32, u32)>;

/// Type alias for page sequences: list of page number sequences
type Sequences = Vec<Vec<u32>>;

/// Example input from the problem statement used for testing and documentation.
pub const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

/// Solves Part 1: Finds sum of middle pages from correctly-ordered sequences.
///
/// Parses the input into rules and sequences, validates each sequence against
/// the precedence rules, and sums the middle page numbers of valid sequences.
///
/// # Parameters
/// * `input` - Multi-line string containing rules and sequences sections
///   separated by blank line
///
/// # Returns
/// Sum of middle page numbers from sequences that respect all ordering rules
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d05::solve_part1;
/// let input = "47|53\n\n75,47,53";
/// assert_eq!(solve_part1(input).unwrap(), 47);
/// ```
pub fn solve_part1(input: &str) -> Result<u32> {
    let (rules, sequences) = parse_input(input)?;

    sequences
        .iter()
        .filter_map(|sequence| {
            is_valid_sequence(sequence, &rules).then_some(get_middle_page(sequence))
        })
        .sum()
}

/// Checks if a sequence is valid according to precedence rules.
///
/// Uses an optimized O(N + M) position-based validation algorithm where N is
/// the number of pages in a sequence and M is the number of rules.
/// This approach builds position maps for first and last occurrences of each
/// page in O(N) time, then validates each rule in constant time O(1), achieving
/// optimal O(N + M) performance.
///
/// For rules with duplicate pages in a sequence, the algorithm correctly
/// handles the constraint that ALL occurrences of the 'before' page must
/// precede ALL occurrences of the 'after' page.
///
/// # Parameters
/// * `sequence` - Vector of page numbers in the order to be validated
/// * `rules` - Vector of (before, after) precedence constraint pairs
///
/// # Returns
/// `true` if sequence respects all applicable ordering rules, `false` otherwise
///
/// # Examples
///
/// ```
/// # use y2024_d05::is_valid_sequence;
/// let rules = vec![(47, 53), (53, 29)];
/// assert!(is_valid_sequence(&[47, 53, 29], &rules));
/// ```
pub fn is_valid_sequence(sequence: &[u32], rules: &[(u32, u32)]) -> bool {
    // Build first and last position maps
    let mut first_pos: FxHashMap<u32, usize> = FxHashMap::default();
    let mut last_pos: FxHashMap<u32, usize> = FxHashMap::default();

    for (i, &page) in sequence.iter().enumerate() {
        first_pos.entry(page).or_insert(i); // Only set if not already set
        last_pos.insert(page, i); // Always update to latest
    }

    // Check each rule: last occurrence of 'before' must precede first
    // occurrence of 'after'
    rules.iter().all(|&(before, after)| {
        last_pos
            .get(&before) // Get last position of 'before' page
            .zip(first_pos.get(&after)) // Combine with first position of 'after' page
            .is_none_or(|(&last_before, &first_after)| last_before < first_after)
        // Rule is satisfied if: both pages missing (None) OR ordering constraint holds
    })
}

/// Naive O(N²M) implementation of Part 1 for performance comparison.
///
/// # Parameters
/// * `input` - Multi-line string containing rules and sequences sections
///   separated by blank line
///
/// # Returns
/// Sum of middle page numbers from sequences that respect all ordering rules
///
/// # Errors
///
/// Returns an error if input parsing fails.
///
/// # Examples
///
/// ```
/// # use y2024_d05::solve_part1_naive;
/// let input = "47|53\n\n75,47,53";
/// assert_eq!(solve_part1_naive(input).unwrap(), 47);
/// ```
pub fn solve_part1_naive(input: &str) -> Result<u32> {
    let (rules, sequences) = parse_input(input)?;

    sequences
        .iter()
        .filter_map(|sequence| {
            is_valid_sequence_naive(sequence, &rules).then_some(get_middle_page(sequence))
        })
        .sum()
}

/// Naive O(N²M) validation algorithm for performance comparison.
///
/// For each pair of elements in the sequence, checks all rules to see if
/// any are violated.
///
/// # Parameters
/// * `sequence` - Vector of page numbers in the order to be validated
/// * `rules` - Vector of (before, after) precedence constraint pairs
///
/// # Returns
/// `true` if sequence respects all applicable ordering rules, `false` otherwise
///
/// # Examples
///
/// ```
/// # use y2024_d05::is_valid_sequence_naive;
/// let rules = vec![(47, 53), (53, 29)];
/// assert!(is_valid_sequence_naive(&[47, 53, 29], &rules));
/// ```
pub fn is_valid_sequence_naive(sequence: &[u32], rules: &[(u32, u32)]) -> bool {
    sequence.iter().tuple_combinations().all(|(&i, &j)| {
        // Since tuple_combinations maintains order, page i always comes before page j
        // Check if any rule says page j should come before page i (violation)
        rules
            .iter()
            .all(|&(before, after)| before != j || after != i)
    })
}

/// Parses input into ordering rules and page sequences.
///
/// Takes input with rules section and sequences section separated by blank
/// line. Rules are in format "X|Y" meaning X must come before Y. Sequences
/// are comma-separated page numbers.
///
/// # Parameters
/// * `input` - Multi-line string with rules and sequences sections
///
/// # Returns
/// Tuple of (ordering_rules, page_sequences) where rules are (before, after)
/// pairs
///
/// # Errors
///
/// Returns an error if:
/// - Input doesn't have exactly 2 sections
/// - Any rule doesn't have exactly 2 parts when split on '|'
/// - Any page number cannot be parsed as u32
///
/// # Examples
///
/// ```
/// # use y2024_d05::parse_input;
/// let input = "47|53\n97|13\n\n75,47,53\n97,13";
/// let (rules, sequences) = parse_input(input).unwrap();
/// assert_eq!(rules, vec![(47, 53), (97, 13)]);
/// ```
pub fn parse_input(input: &str) -> Result<(Rules, Sequences)> {
    // Parse input into exactly 2 sections: rules and sequences

    let (rules_section, sequences_section) = input
        .split("\n\n")
        .map(|section| section.trim())
        .filter(|section| !section.is_empty())
        .collect_tuple()
        .context("Input must have exactly 2 sections")?;

    // Parse rules: "X|Y" format with whitespace tolerance

    let rules = rules_section
        .lines()
        .map(|line| {
            line.split('|')
                .map(str::trim)
                .collect_tuple()
                .context(format!("Rule must have format 'X|Y', found: {line}"))
                .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
        })
        .try_collect()?;

    // Parse sequences: comma-separated page numbers with whitespace tolerance

    let sequences = sequences_section
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().parse()).collect())
        .try_collect()?;

    Ok((rules, sequences))
}

/// Gets the middle page number from a sequence.
///
/// For sequences with odd length, returns the true middle element.
/// For sequences with even length, returns the element at index len/2
/// (which is the second of the two middle elements).
///
/// # Parameters
/// * `sequence` - Vector of page numbers from which to extract middle page
///
/// # Returns
/// The middle page number
///
/// # Errors
///
/// Returns an error if the sequence is empty.
///
/// # Examples
///
/// ```
/// # use y2024_d05::get_middle_page;
/// assert_eq!(get_middle_page(&[75, 47, 61, 53, 29]).unwrap(), 61);
/// ```
pub fn get_middle_page(sequence: &[u32]) -> Result<u32> {
    sequence
        .get(sequence.len() / 2)
        .copied() // Convert &u32 to u32
        .context("Cannot get middle page of empty sequence")
}
