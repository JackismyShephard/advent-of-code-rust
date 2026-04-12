use rstest::rstest;
use y2024_d03::{
    extract_enabled_mul_instructions, extract_mul_instructions, solve_part1, solve_part2,
    EXAMPLE_INPUT, EXAMPLE_INPUT_PART2,
};

// ===== CORE FUNCTION TESTS =====

#[test]
fn test_extract_mul_instructions_example() {
    let instructions = extract_mul_instructions(EXAMPLE_INPUT).unwrap();
    assert_eq!(instructions, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
}

#[rstest]
#[case(EXAMPLE_INPUT, vec![(2, 4), (5, 5), (11, 8), (8, 5)])] // Example input
#[case("mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 ) mul[3,7] mul(123,456)", vec![(123, 456)])] // Invalid formats ignored
#[case("", vec![])] // Empty input
#[case("no mul instructions here", vec![])] // No valid instructions
#[case("mul(1,2) mul(12,34) mul(123,456) mul(1234,5) mul(1,2345)", vec![(1, 2), (12, 34), (123, 456)])] // 1-3 digit boundary
#[case("mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 ) mul[3,7] mul(123,456)", vec![(123, 456)])] // Invalid formats ignored
#[case("", vec![])] // Empty input
#[case("no mul instructions here", vec![])] // No valid instructions
#[case("mul(1,2) mul(12,34) mul(123,456) mul(1234,5) mul(1,2345)", vec![(1, 2), (12, 34), (123, 456)])] // 1-3 digit boundary
fn test_extract_mul_instructions_edge_cases(
    #[case] input: &str,
    #[case] expected: Vec<(u32, u32)>,
) {
    let instructions = extract_mul_instructions(input).unwrap();
    assert_eq!(instructions, expected);
}

#[rstest]
#[case("", vec![])] // Empty input
#[case("no mul instructions here", vec![])] // No valid instructions
#[case("mul(4* mul[3,7] mul ( 2 , 4 )", vec![])] // All invalid formats
fn test_extract_mul_instructions_error_cases(
    #[case] input: &str,
    #[case] expected: Vec<(u32, u32)>,
) {
    let result = extract_mul_instructions(input);
    assert!(
        result.is_ok(),
        "Function should handle invalid input gracefully"
    );
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_extract_enabled_mul_instructions_examples() {
    let instructions = extract_enabled_mul_instructions(EXAMPLE_INPUT_PART2).unwrap();
    assert_eq!(instructions, vec![(2, 4), (8, 5)]);
}

#[rstest]
#[case("mul(3,4)don't()mul(5,6)", vec![(3, 4)])] // Instructions enabled by default
#[case("mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)", vec![(1, 1), (3, 3), (5, 5)])] // Multiple state changes
#[case("mul(1,2)mul(3,4)mul(5,6)", vec![(1, 2), (3, 4), (5, 6)])] // No state changes
#[case("don't()mul(1,2)mul(3,4)mul(5,6)", vec![])] // All disabled
#[case("", vec![])] // Empty input
fn test_extract_enabled_mul_instructions_edge_cases(
    #[case] input: &str,
    #[case] expected: Vec<(u32, u32)>,
) {
    let instructions = extract_enabled_mul_instructions(input).unwrap();
    assert_eq!(instructions, expected);
}

#[rstest]
#[case("", vec![])] // Empty input
#[case("don't()mul(1,2)", vec![])] // Disabled instructions
#[case("invalid input", vec![])] // No valid instructions
fn test_extract_enabled_mul_instructions_error_cases(
    #[case] input: &str,
    #[case] expected: Vec<(u32, u32)>,
) {
    let result = extract_enabled_mul_instructions(input);
    assert!(
        result.is_ok(),
        "Function should handle invalid input gracefully"
    );
    assert_eq!(result.unwrap(), expected);
}

// ===== SOLVE FUNCTION TESTS =====

#[rstest]
#[case(solve_part1, EXAMPLE_INPUT, 161)] // Part 1 with example input
#[case(solve_part2, EXAMPLE_INPUT_PART2, 48)] // Part 2 with example input
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> anyhow::Result<u32>,
    #[case] input: &str,
    #[case] expected: u32,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("mul(2,3)", 6)] // Simple multiplication
#[case("mul(10,10)", 100)] // Two-digit numbers
#[case("mul(1,1)mul(2,2)mul(3,3)", 14)] // Multiple instructions: 1 + 4 + 9 = 14
#[case("no valid instructions", 0)] // No valid instructions
#[case("mul(4* mul[3,7] mul ( 2 , 4 )", 0)] // Invalid format instructions
fn test_solve_functions_edge_cases(
    #[values(solve_part1, solve_part2)] solve_fn: fn(&str) -> anyhow::Result<u32>,
    #[case] input: &str,
    #[case] expected: u32,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected, "Failed for input: {input:?}");
}

#[rstest]
#[case("don't()mul(2,3)", 0)] // Simple disabled case
#[case("don't()mul(2,3)do()mul(4,5)", 20)] // Re-enabled case
#[case("mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)", 35)] // Complex state changes: 1*1 + 3*3 + 5*5 = 35
fn test_solve_part2_control_flow(#[case] input: &str, #[case] expected: u32) {
    let result = solve_part2(input).unwrap();
    assert_eq!(result, expected, "Part 2 failed for input: {input:?}");
}

#[rstest]
#[case(solve_part1, 167650499)] // Part 1 with real input
#[case(solve_part2, 95846796)] // Part 2 with real input
fn test_solve_functions_real_input(
    #[case] solve_fn: fn(&str) -> anyhow::Result<u32>,
    #[case] expected: u32,
) {
    let input =
        shared::read_local_input!().expect("Failed to read input.txt - make sure it exists");
    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
