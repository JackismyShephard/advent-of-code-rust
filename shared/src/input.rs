//! Shared input reading and parsing utilities for Advent of Code challenges.
//!
//! This module provides common functionality for reading puzzle input files
//! and parsing them into commonly used formats across different days.

use anyhow::Result;
use std::fs;

/// Reads the puzzle input file for a specific Advent of Code day.
///
/// Constructs the standard input file path and reads the entire file contents
/// into memory as a UTF-8 string. Follows the repository naming convention
/// `<year>/dayXX/input.txt`.
///
/// # Parameters
/// * `day` - The day number (1-25) for which to read the input file
///
/// # Returns
/// Complete file contents as a UTF-8 string with original formatting preserved
///
/// # Errors
///
/// Returns an error if:
/// - The input file doesn't exist at the expected path
/// - File cannot be read due to permissions or I/O errors
/// - File contains invalid UTF-8 sequences
///
/// # Examples
///
/// ```
/// # use shared::input::read_input;
/// # use std::fs;
/// # use std::io::Write;
/// # // Create a test input file
/// # fs::create_dir_all("2024/day01").unwrap();
/// # let mut file = fs::File::create("2024/day01/input.txt").unwrap();
/// # writeln!(file, "123\n456").unwrap();
/// let input = read_input(2024, 1)?;
/// assert_eq!(input, "123\n456\n");
/// # fs::remove_file("2024/day01/input.txt").unwrap();
/// # fs::remove_dir("2024/day01").unwrap();
/// # fs::remove_dir("2024").unwrap();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn read_input(year: u16, day: u8) -> Result<String> {
    let filename = format!("{year}/day{day:02}/input.txt");
    Ok(fs::read_to_string(filename)?)
}

/// Reads `input.txt` from the calling crate's directory.
///
/// This is intended for day crates that keep their puzzle input next to their
/// own `Cargo.toml`. It must be a macro so `CARGO_MANIFEST_DIR` resolves in
/// the calling crate rather than in `shared`.
#[macro_export]
macro_rules! read_local_input {
    () => {
        std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt"))
    };
}
