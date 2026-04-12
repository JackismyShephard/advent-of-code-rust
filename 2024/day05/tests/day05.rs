use rstest::rstest;
use y2024_d05::{
    get_middle_page, is_valid_sequence, is_valid_sequence_naive, parse_input, solve_part1,
    solve_part1_naive, EXAMPLE_INPUT,
};

// Type alias for validator function to avoid clippy complexity warnings
type Validator = fn(&[u32], &[(u32, u32)]) -> bool;

// ===== PARSE INPUT TESTS =====

#[test]
fn test_parse_input_example() {
    let (rules, sequences) = parse_input(EXAMPLE_INPUT).unwrap();
    assert_eq!(rules.len(), 21);
    assert_eq!(sequences.len(), 6);
    assert_eq!(rules[0], (47, 53));
    assert_eq!(sequences[0], vec![75, 47, 61, 53, 29]);
}

#[rstest]
#[case("47|53\n\n75,47,53", vec![(47, 53)], vec![vec![75, 47, 53]])] // basic case
#[case("47 | 53\n\n75, 47, 53", vec![(47, 53)], vec![vec![75, 47, 53]])] // whitespace handling
#[case("  47|53  \n\n  75,47,53  ", vec![(47, 53)], vec![vec![75, 47, 53]])] // leading/trailing whitespace
#[case("47|53\n\n75,47,53\n", vec![(47, 53)], vec![vec![75, 47, 53]])] // trailing newline
fn test_parse_input_valid_cases(
    #[case] input: &str,
    #[case] expected_rules: Vec<(u32, u32)>,
    #[case] expected_sequences: Vec<Vec<u32>>,
) {
    let (rules, sequences) = parse_input(input).unwrap();
    assert_eq!(rules, expected_rules, "Rules mismatch for input: {input:?}");
    assert_eq!(
        sequences, expected_sequences,
        "Sequences mismatch for input: {input:?}"
    );
}

#[rstest]
// Wrong number of sections
#[case("", "Input must have exactly 2 sections")] // empty input
#[case("47|53", "Input must have exactly 2 sections")] // only one section
#[case("47|53\n\n75,47\n\nextra", "Input must have exactly 2 sections")] // three sections
// Invalid rule formats
#[case("47|\n\n75,47", "cannot parse integer from empty string")] // missing after value
#[case("47\n\n75,47", "Rule must have format 'X|Y', found: 47")] // no pipe separator
#[case("47|53|61\n\n75,47", "Rule must have format 'X|Y', found: 47|53|61")] // too many parts
fn test_parse_input_error_cases_with_messages(#[case] input: &str, #[case] expected_error: &str) {
    let result = parse_input(input);
    assert!(result.is_err(), "Expected error for input: {input:?}");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains(expected_error),
        "Expected error '{expected_error}' for input {input:?}, got: {error_msg}"
    );
}

#[rstest]
// Numeric parsing errors (standard library error messages)
#[case("abc|53\n\n75,47")] // non-numeric before value
#[case("47|def\n\n75,47")] // non-numeric after value
#[case("47|53\n\n75,abc")] // non-numeric page in sequence
fn test_parse_input_numeric_error_cases(#[case] input: &str) {
    let result = parse_input(input);
    assert!(
        result.is_err(),
        "Expected numeric parsing error for input: {input:?}"
    );

    // These will have parse error messages from the standard library
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("invalid digit") || error_msg.contains("ParseIntError"),
        "Expected numeric parsing error for input {input:?}, got: {error_msg}"
    );
}

// ===== CORE FUNCTION TESTS =====

#[rstest]
#[case(&[75, 47, 61, 53, 29], 61)] // odd length (5) -> index 2
#[case(&[1, 2, 3, 4], 3)] // even length (4) -> index 2 (upper middle)
#[case(&[1, 2], 2)] // even length (2) -> index 1
#[case(&[42], 42)] // single element -> index 0
fn test_get_middle_page(#[case] sequence: &[u32], #[case] expected: u32) {
    assert_eq!(
        get_middle_page(sequence).unwrap(),
        expected,
        "Failed for sequence {sequence:?}"
    );
}

#[test]
fn test_get_middle_page_empty() {
    let result = get_middle_page(&[]);
    assert!(result.is_err(), "Empty sequence should return error");

    let error_msg = result.unwrap_err().to_string();
    assert_eq!(
        error_msg, "Cannot get middle page of empty sequence",
        "Expected specific error message for empty sequence"
    );
}

#[rstest]
#[case(&[75, 47, 61, 53, 29], &[(47, 53), (53, 29)], true)] // valid order
#[case(&[53, 47, 61, 29], &[(47, 53), (53, 29)], false)] // violates 47|53
#[case(&[47, 29, 53], &[(47, 53), (53, 29)], false)] // violates 53|29
#[case(&[47], &[(47, 53)], true)] // single element, rule not applicable
#[case(&[], &[(47, 53)], true)] // empty sequence
#[case(&[1, 2, 3], &[], true)] // no rules - always valid
#[case(&[47, 53, 29], &[(47, 53), (53, 29), (47, 29)], true)] // transitive constraints
fn test_is_valid_sequence(
    #[values(is_valid_sequence, is_valid_sequence_naive)] validator: Validator,
    #[case] sequence: &[u32],
    #[case] rules: &[(u32, u32)],
    #[case] expected: bool,
) {
    assert_eq!(
        validator(sequence, rules),
        expected,
        "Algorithm failed for sequence {sequence:?} with rules {rules:?}"
    );
}
#[rstest]
// Interleaved duplicates - should fail
#[case(&[1, 2, 1], &[(1, 2)], false)] // XYX pattern with X|Y rule
#[case(&[1, 2, 1], &[(2, 1)], false)] // XYX pattern with Y|X rule
#[case(&[2, 1, 2], &[(1, 2)], false)] // YXY pattern with X|Y rule
#[case(&[2, 1, 2], &[(2, 1)], false)] // YXY pattern with Y|X rule
#[case(&[1, 2, 1, 2], &[(1, 2)], false)] // XYXY pattern with X|Y rule
#[case(&[2, 1, 2, 1], &[(2, 1)], false)] // YXYX pattern with Y|X rule
// Grouped duplicates - should pass
#[case(&[1, 1, 2, 2], &[(1, 2)], true)] // XXYY pattern with X|Y rule
#[case(&[2, 2, 1, 1], &[(2, 1)], true)] // YYXX pattern with Y|X rule
// Missing pages - should pass
#[case(&[1, 1, 1], &[(1, 2)], true)] // XXX pattern with X|Y rule (Y missing)
#[case(&[2, 2, 2], &[(1, 2)], true)] // YYY pattern with X|Y rule (X missing)
// With other pages
#[case(&[1, 3, 2, 1], &[(1, 2)], false)] // XZYX pattern with X|Y rule - should fail
#[case(&[1, 3, 2, 3], &[(1, 2)], true)] // XZYZ pattern with X|Y rule - should pass
fn test_is_valid_sequence_duplicates(
    #[values(is_valid_sequence, is_valid_sequence_naive)] validator: Validator,
    #[case] sequence: &[u32],
    #[case] rules: &[(u32, u32)],
    #[case] expected: bool,
) {
    assert_eq!(
        validator(sequence, rules),
        expected,
        "Algorithm failed for sequence {sequence:?} with rules {rules:?}"
    );
}

#[rstest]
// Simple chain with transitive constraints: 1->2->3, 1->3
#[case(&[1, 2, 3], &[(1, 2), (2, 3), (1, 3)], true)] // valid chain order
#[case(&[1, 3], &[(1, 2), (2, 3), (1, 3)], true)] // valid with missing middle
#[case(&[2, 1, 3], &[(1, 2), (2, 3), (1, 3)], false)] // violates 1->2
#[case(&[3, 2, 1], &[(1, 2), (2, 3), (1, 3)], false)]
// completely reversed
// Complex cross-chain dependencies: two chains 1->2->3 and 4->5->6 with cross-links
#[case(
    &[1, 2, 3, 4, 5, 6],
    &[(1, 2), (2, 3), (1, 3), (4, 5), (5, 6), (4, 6), (1, 4), (2, 5), (3, 6)],
    true
)] // perfect order
#[case(
    &[1, 2, 4, 3, 5, 6],
    &[(1, 2), (2, 3), (1, 3), (4, 5), (5, 6), (4, 6), (1, 4), (2, 5), (3, 6)],
    true
)] // valid interleaving
#[case(
    &[1, 4, 2, 5, 3, 6],
    &[(1, 2), (2, 3), (1, 3), (4, 5), (5, 6), (4, 6), (1, 4), (2, 5), (3, 6)],
    true
)] // complex valid interleaving
#[case(
    &[2, 1, 3, 4, 5, 6],
    &[(1, 2), (2, 3), (1, 3), (4, 5), (5, 6), (4, 6), (1, 4), (2, 5), (3, 6)],
    false
)] // violates 1->2
#[case(
    &[1, 2, 3, 5, 4, 6],
    &[(1, 2), (2, 3), (1, 3), (4, 5), (5, 6), (4, 6), (1, 4), (2, 5), (3, 6)],
    false
)] // violates 4->5
#[case(
    &[4, 1, 2, 3, 5, 6],
    &[(1, 2), (2, 3), (1, 3), (4, 5), (5, 6), (4, 6), (1, 4), (2, 5), (3, 6)],
    false
)] // violates 1->4
// Multiple paths to same node: both 1->3 and 2->3
#[case(&[1, 2, 3], &[(1, 3), (2, 3)], true)] // both before 3
#[case(&[2, 1, 3], &[(1, 3), (2, 3)], true)] // both before 3, different order
#[case(&[3, 1, 2], &[(1, 3), (2, 3)], false)] // 3 comes first
fn test_is_valid_sequence_complex(
    #[values(is_valid_sequence, is_valid_sequence_naive)] validator: Validator,
    #[case] sequence: &[u32],
    #[case] rules: &[(u32, u32)],
    #[case] expected: bool,
) {
    assert_eq!(
        validator(sequence, rules),
        expected,
        "Algorithm failed for sequence {sequence:?} with rules {rules:?}"
    );
}

// ===== SOLVE FUNCTION TESTS  =====

#[rstest]
#[case(solve_part1_naive, 143)] // Naive solve function
#[case(solve_part1, 143)] // Optimized solve function
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> Result<u32, anyhow::Error>,
    #[case] expected: u32,
) {
    let result = solve_fn(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
// Basic edge cases
#[case("47|53\n\n75,47,53", 47)] // single valid sequence
#[case("47|53\n\n53,47", 0)] // single invalid sequence
#[case("47|53\n\n75,47,53\n53,47", 47)] // mixed valid/invalid
#[case("47|53\n\n1,2,3", 2)] // rule doesn't apply to sequence
// Additional edge cases
#[case("47|53\n53|29\n\n75,47,53,29\n1,2,3", 55)] // multiple rules: 53 + 2 = 55
#[case("1|2\n2|3\n3|4\n\n1,2,3,4\n4,3,2,1", 3)] // transitive rules, one valid: middle=3
#[case("1|2\n\n1\n2\n1,2", 5)] // single-element sequences: 1+2+2=5
#[case("1|2\n2|3\n\n1,2,3,4,5", 3)] // rules subset of sequence
fn test_solve_functions(
    #[values(solve_part1, solve_part1_naive)] solver: fn(&str) -> Result<u32, anyhow::Error>,
    #[case] input: &str,
    #[case] expected: u32,
) {
    assert_eq!(
        solver(input).unwrap(),
        expected,
        "Solver failed for input: {input:?}"
    );
}

#[rstest]
#[case(solve_part1_naive, 4578)] // Naive solve function
#[case(solve_part1, 4578)] // Optimized solve function
fn test_solve_functions_with_real_input(
    #[case] solve_fn: fn(&str) -> Result<u32, anyhow::Error>,
    #[case] expected: u32,
) {
    let input =
        shared::read_local_input!().expect("Failed to read input.txt - make sure it exists");

    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
