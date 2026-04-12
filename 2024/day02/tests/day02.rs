use rstest::rstest;
use y2024_d02::{
    is_safe, is_safe_with_dampener, parse_input, solve_part1, solve_part2, EXAMPLE_INPUT,
};

// ===== PARSE INPUT TESTS =====

#[test]
fn test_parse_input_example() {
    let input = "1 2 3\n4 5 6";
    let reports = parse_input(input).unwrap();
    assert_eq!(reports, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[rstest]
#[case("", vec![], "empty input")] // Empty input
#[case("1", vec![vec![1]], "single number")] // Single number on line
#[case("1  2  3", vec![vec![1, 2, 3]], "extra spaces")] // Extra spaces handled
#[case("1 2\n3 4 5", vec![vec![1, 2], vec![3, 4, 5]], "different line lengths")] // Lines with different lengths
#[case("1 2 3\n\n4 5", vec![vec![1, 2, 3], vec![4, 5]], "empty line")] // Empty line filtered out
#[case("1\n2 3\n4 5 6 7", vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]], "mixed lengths")] // Mixed line lengths
fn test_parse_input_edge_cases(
    #[case] input: &str,
    #[case] expected: Vec<Vec<i32>>,
    #[case] description: &str,
) {
    let reports = parse_input(input).unwrap();
    assert_eq!(reports, expected, "Failed for {description}");
}

#[rstest]
#[case("1 abc 3", "invalid digit")] // Non-numeric values
#[case("1 2 3\n4 def 6", "invalid digit")] // Mixed valid/invalid lines
#[case("abc def ghi", "invalid digit")] // All non-numeric
#[case("1 2\n\n3 xyz", "invalid digit")] // Error after empty line
fn test_parse_input_errors(#[case] input: &str, #[case] expected_error: &str) {
    let result = parse_input(input);
    assert!(result.is_err(), "Should error on input: {input:?}");
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains(expected_error),
        "Error message should contain '{expected_error}', got: {error}"
    );
}

// ===== CORE FUNCTION TESTS =====

#[rstest]
#[case(&[7, 6, 4, 2, 1], true)] // Safe: decreasing by 1 or 2
#[case(&[1, 2, 7, 8, 9], false)] // Unsafe: 2->7 is increase of 5
#[case(&[9, 7, 6, 2, 1], false)] // Unsafe: 6->2 is decrease of 4
#[case(&[1, 3, 2, 4, 5], false)] // Unsafe: 1->3 increasing, 3->2 decreasing
#[case(&[8, 6, 4, 4, 1], false)] // Unsafe: 4->4 no change
#[case(&[1, 3, 6, 7, 9], true)] // Safe: increasing by 1, 2, or 3
fn test_is_safe_examples(#[case] levels: &[i32], #[case] expected: bool) {
    assert_eq!(is_safe(levels), expected);
}

#[rstest]
#[case(&[], true)] // Empty report is safe
#[case(&[1], true)] // Single level is safe
#[case(&[1, 2], true)] // Two levels, valid difference
#[case(&[1, 5], false)] // Two levels, invalid difference (4)
#[case(&[5, 5], false)] // Two levels, no change
fn test_is_safe_edge_cases(#[case] levels: &[i32], #[case] expected: bool) {
    assert_eq!(is_safe(levels), expected);
}

#[rstest]
#[case(&[7, 6, 4, 2, 1], true)] // Safe without removing any level
#[case(&[1, 2, 7, 8, 9], false)] // Unsafe regardless of removal
#[case(&[9, 7, 6, 2, 1], false)] // Unsafe regardless of removal
#[case(&[1, 3, 2, 4, 5], true)] // Safe by removing second level (3)
#[case(&[8, 6, 4, 4, 1], true)] // Safe by removing third level (4)
#[case(&[1, 3, 6, 7, 9], true)] // Safe without removing any level
fn test_is_safe_with_dampener_examples(#[case] levels: &[i32], #[case] expected: bool) {
    assert_eq!(is_safe_with_dampener(levels), expected);
}

#[rstest]
#[case(&[], true)] // Empty report is safe
#[case(&[1], true)] // Single level is safe
#[case(&[1, 2], true)] // Two levels, valid difference
#[case(&[1, 5], true)] // Two levels, can remove one to make safe
#[case(&[5, 5], true)] // Two levels, can remove one to make safe
#[case(&[1, 2, 3], true)] // Already safe
#[case(&[1, 4, 3], true)] // Can remove 4 to make 1->3 safe
#[case(&[1, 5, 9, 13], false)] // All jumps too large, can't fix with one removal
fn test_is_safe_with_dampener_edge_cases(#[case] levels: &[i32], #[case] expected: bool) {
    assert_eq!(is_safe_with_dampener(levels), expected);
}

// ===== SOLVE FUNCTION TESTS =====

#[rstest]
#[case(solve_part1, 2)] // Part 1 imperative with example input
#[case(solve_part2, 4)] // Part 2 with example input
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> anyhow::Result<usize>,
    #[case] expected: usize,
) {
    let result = solve_fn(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, "1 2 3\n5 4 3 2\n1 1 1", 2)] // Part 1: First two safe, third has no changes
#[case(solve_part2, "1 2 3\n1 5 2\n10 8 6 4\n1 1 1 1", 3)] // Part 2: Custom dampener test
fn test_solve_functions_edge_cases(
    #[case] solve_fn: fn(&str) -> anyhow::Result<usize>,
    #[case] input: &str,
    #[case] expected: usize,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, 686)] // Part 1 imperative with real input
#[case(solve_part2, 717)] // Part 2 with real input
fn test_solve_functions_real_input(
    #[case] solve_fn: fn(&str) -> anyhow::Result<usize>,
    #[case] expected: usize,
) {
    let input =
        shared::read_local_input!().expect("Failed to read input.txt - make sure it exists");
    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
