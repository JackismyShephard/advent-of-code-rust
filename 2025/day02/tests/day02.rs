use y2025_d02::{solve_part1, solve_part2, EXAMPLE_INPUT};

// ===== SOLVE FUNCTION TESTS =====

#[test]
fn test_solve_part1_reports_unimplemented() {
    let error = solve_part1(EXAMPLE_INPUT).unwrap_err();
    assert_eq!(error.to_string(), "Part 1 not implemented yet");
}

#[test]
fn test_solve_part2_reports_unimplemented() {
    let error = solve_part2(EXAMPLE_INPUT).unwrap_err();
    assert_eq!(error.to_string(), "Part 2 not implemented yet");
}
