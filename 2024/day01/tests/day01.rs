use rstest::rstest;
use y2024_d01::{parse_input, solve_part1, solve_part2, solve_part2_naive, EXAMPLE_INPUT};

// ===== PARSE INPUT TESTS =====

#[test]
fn test_parse_input_example() {
    let (left, right) = parse_input(EXAMPLE_INPUT).unwrap();
    assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
    assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
}

#[rstest]
#[case("1  2", vec![1], vec![2], "extra spaces")] // Extra spaces are handled
#[case("1 2\n\n3 4", vec![1, 3], vec![2, 4], "empty lines ignored")] // Empty lines are skipped
#[case("\n\n1 2\n3 4\n\n", vec![1, 3], vec![2, 4], "leading/trailing whitespace ignored")] // Leading/trailing empty lines
#[case("1 2\n   \n3 4", vec![1, 3], vec![2, 4], "whitespace-only lines ignored")] // Whitespace-only lines
#[case("", vec![], vec![], "empty input")] // Empty input returns empty vectors
#[case("\n\n   \n", vec![], vec![], "only whitespace")] // Only whitespace returns empty vectors
fn test_parse_input_edge_cases(
    #[case] input: &str,
    #[case] expected_left: Vec<i32>,
    #[case] expected_right: Vec<i32>,
    #[case] description: &str,
) {
    let (left, right) = parse_input(input).unwrap();
    assert_eq!(left, expected_left, "Left mismatch for {description}");
    assert_eq!(right, expected_right, "Right mismatch for {description}");
}

#[rstest]
#[case("1", "exactly two")] // Single number
#[case("1 2 3", "exactly two")] // Too many numbers
#[case("1 2\n3", "exactly two")] // Mixed valid and invalid lines
fn test_parse_input_errors(#[case] input: &str, #[case] expected_error: &str) {
    let result = parse_input(input);
    assert!(result.is_err(), "Should error on input: {input:?}");
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains(expected_error),
        "Error message should contain '{expected_error}', got: {error}"
    );
}

// ===== SOLVE FUNCTION TESTS =====

#[rstest]
#[case(solve_part1, 11)] // Part 1 with example input
#[case(solve_part2, 31)] // Part 2 with example input
#[case(solve_part2_naive, 31)] // Part 2 naive with example input
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> anyhow::Result<i32>,
    #[case] expected: i32,
) {
    let result = solve_fn(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("1 2\n3 4", solve_part1, 2)] // Simple case: sorted [1,3] and [2,4] -> |1-2| + |3-4| = 1 + 1 = 2
#[case("1 2\n3 4", solve_part2, 0)] // No similarity (no common numbers)
#[case("1 2\n3 4", solve_part2_naive, 0)] // Same as above
#[case("", solve_part1, 0)] // Empty input edge case
#[case("", solve_part2, 0)] // Empty input edge case
#[case("", solve_part2_naive, 0)] // Empty input edge case
fn test_solve_functions_edge_cases(
    #[case] input: &str,
    #[case] solve_fn: fn(&str) -> anyhow::Result<i32>,
    #[case] expected: i32,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, 1603498)] // Part 1 with real input
#[case(solve_part2, 25574739)] // Part 2 with real input
#[case(solve_part2_naive, 25574739)] // Part 2 naive with real input
fn test_solve_functions_real_input(
    #[case] solve_fn: fn(&str) -> anyhow::Result<i32>,
    #[case] expected: i32,
) {
    let input =
        shared::read_local_input!().expect("Failed to read input.txt - make sure it exists");
    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
