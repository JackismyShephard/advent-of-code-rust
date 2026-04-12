//! Day 4: Ceres Search
//!
//! Solution for Advent of Code 2024 Day 4.
//!
//! Part 1: Find all occurrences of "XMAS" in a 2D word search grid.
//! The word can appear horizontally, vertically, or diagonally, and can be
//! written forwards or backwards.
//!
//! Part 2: Find all X-MAS patterns in a 2D word search grid.
//! An X-MAS pattern consists of two "MAS" words that intersect at their center 'A'
//! to form an X shape. Each "MAS" can be written forwards or backwards ("SAM").

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

/// Solves Part 1: Finds all occurrences of "XMAS" in the word search grid.
///
/// Searches through every position in the grid and counts how many times
/// "XMAS" appears in all 8 directions (horizontal, vertical, and
/// diagonal). Words can be written forwards or backwards.
///
/// # Parameters
/// * `input` - Multi-line string containing the character grid
///
/// # Returns
/// Total number of "XMAS" occurrences found in the grid
///
/// # Examples
///
/// ```
/// # use y2024_d04::solve_part1;
/// let input = "XMAS\nMASX";
/// assert_eq!(solve_part1(input), 1); // "XMAS" going right from (0,0)
/// ```
pub fn solve_part1(input: &str) -> usize {
    let grid = parse_input(input);

    (0..grid.len())
        .map(|row| {
            (0..grid[row].len())
                .map(|col| count_xmas_at_position(&grid, row, col))
                .sum::<usize>()
        })
        .sum()
}

/// Counts the number of times "XMAS" appears starting from a specific
/// position.
///
/// Checks all 8 directions from the given position and counts how many times
/// the target word "XMAS" appears.
///
/// # Parameters
/// * `grid` - The 2D character grid to search in
/// * `row` - Row position to start searching from (0-indexed)
/// * `col` - Column position to start searching from (0-indexed)
///
/// # Returns
/// Number of times "XMAS" appears starting from this position (0-8)
///
/// # Examples
///
/// ```
/// # use y2024_d04::{parse_input, count_xmas_at_position};
/// let grid = parse_input("XMAS\nMASX");
/// assert_eq!(count_xmas_at_position(&grid, 0, 0), 1); // "XMAS" right
/// ```
pub fn count_xmas_at_position(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1),  // up-right
    ];

    DIRECTIONS
        .iter()
        .filter(|&&(row_delta, col_delta)| check_direction(grid, row, col, row_delta, col_delta))
        .count()
}

/// Checks if "XMAS" appears in a specific direction in the given grid.
///
/// Starting from the given position, checks if the characters in the specified
/// direction match the target word "XMAS". Returns false if the word extends
/// beyond grid boundaries.
///
/// # Parameters
/// * `grid` - The 2D character grid to search in
/// * `start_row` - Starting row position (0-indexed)
/// * `start_col` - Starting column position (0-indexed)
/// * `row_delta` - Row direction (-1, 0, or 1)
/// * `col_delta` - Column direction (-1, 0, or 1)
///
/// # Returns
/// `true` if "XMAS" is found in the specified direction, `false` otherwise
///
/// # Examples
///
/// ```
/// # use y2024_d04::{parse_input, check_direction};
/// let grid = parse_input("XMAS\nABCD");
/// assert!(check_direction(&grid, 0, 0, 0, 1)); // "XMAS" going right
/// assert!(!check_direction(&grid, 0, 0, 1, 0)); // "XABC" going down
/// ```
pub fn check_direction(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    row_delta: isize,
    col_delta: isize,
) -> bool {
    const XMAS_CHARS: &[char] = &['X', 'M', 'A', 'S'];

    XMAS_CHARS.iter().enumerate().all(|(i, &target_char)| {
        let target_row = start_row as isize + (i as isize * row_delta);
        let target_col = start_col as isize + (i as isize * col_delta);
        char_matches_at(grid, target_row, target_col, target_char)
    })
}
/// Solves Part 2: Finds all X-MAS patterns in the given grid.
///
/// Searches for patterns where two "MAS" words intersect at their center
/// 'A' to form an X shape. Each "MAS" can be written forwards or backwards.
///
/// # Parameters
/// * `input` - Multi-line string containing the character grid
///
/// # Returns
/// Total number of X-MAS patterns found in the grid
///
/// # Examples
/// ```
/// # use y2024_d04::solve_part2;
/// let input = "M.S\n.A.\nM.S";
/// assert_eq!(solve_part2(input), 1);
/// ```
pub fn solve_part2(input: &str) -> usize {
    let grid = parse_input(input);

    (0..grid.len())
        .map(|row| {
            (0..grid[row].len())
                .filter(|&col| is_xmas_pattern(&grid, row, col))
                .count()
        })
        .sum()
}

/// Checks if a 3x3 region centered at the given position contains an X-MAS
/// pattern.
///
/// An X-MAS pattern consists of two "MAS" words that intersect at the center
/// 'A' to form an X shape. Each "MAS" can be written forwards or backwards
/// ("SAM").
///
/// # Parameters
/// * `grid` - The 2D character grid to check
/// * `center_row` - Row position of the center 'A' (0-indexed)
/// * `center_col` - Column position of the center 'A' (0-indexed)
///
/// # Returns
/// `true` if a valid X-MAS pattern is found, `false` otherwise
///
/// # Examples
/// ```
/// # use y2024_d04::{parse_input, is_xmas_pattern};
/// let grid = parse_input("M.S\n.A.\nM.S");
/// assert!(is_xmas_pattern(&grid, 1, 1)); // X-MAS pattern at center
/// ```
pub fn is_xmas_pattern(grid: &[Vec<char>], center_row: usize, center_col: usize) -> bool {
    const MAS_PATTERN: [char; 3] = ['M', 'A', 'S'];
    const SAM_PATTERN: [char; 3] = ['S', 'A', 'M'];

    let center_row_signed = center_row as isize;
    let center_col_signed = center_col as isize;

    // Both diagonal directions: top-left to bottom-right (1,1) and
    // top-right to bottom-left (1,-1)
    let diagonal_directions = [(1, 1), (1, -1)];
    let patterns = [&MAS_PATTERN, &SAM_PATTERN];

    diagonal_directions.iter().all(|&(row_delta, col_delta)| {
        // Check if this diagonal contains either "MAS" or "SAM"
        patterns.iter().any(|&pattern| {
            // Check if pattern matches in this diagonal direction
            pattern.iter().enumerate().all(|(i, &expected_char)| {
                let relative_pos = i as isize - 1; // -1, 0, 1 for positions relative to center
                let row = center_row_signed + (relative_pos * row_delta);
                let col = center_col_signed + (relative_pos * col_delta);
                char_matches_at(grid, row, col, expected_char)
            })
        })
    })
}

/// Checks if a character at the specified position matches the expected
/// character.
///
/// Performs bounds checking and returns false if the position is out of bounds
/// or if the character doesn't match.
///
/// # Parameters
/// * `grid` - The 2D character grid to access
/// * `row` - Row position (can be negative, will return false if out of
///   bounds)
/// * `col` - Column position (can be negative, will return false if out of
///   bounds)
/// * `expected` - The character to check for at this position
///
/// # Returns
/// `true` if the position is valid and contains the expected character,
/// `false` otherwise
fn char_matches_at(grid: &[Vec<char>], row: isize, col: isize, expected: char) -> bool {
    if row < 0 || col < 0 {
        return false;
    }

    let row = row as usize;
    let col = col as usize;

    if row >= grid.len() || col >= grid[row].len() {
        return false;
    }

    grid[row][col] == expected
}

/// Parses the input string into a 2D grid of characters.
///
/// Takes the input text and converts it into a vector of character vectors,
/// where each inner vector represents a row in the grid.
///
/// # Parameters
/// * `input` - Multi-line string containing the character grid
///
/// # Returns
/// 2D vector of characters representing the grid
///
/// # Examples
///
/// ```
/// # use y2024_d04::parse_input;
/// let input = "ABC\nDEF";
/// let grid = parse_input(input);
/// assert_eq!(grid, vec![
///     vec!['A', 'B', 'C'],
///     vec!['D', 'E', 'F']
/// ]);
/// ```
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}
