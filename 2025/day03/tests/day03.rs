use y2025_d03::{EXAMPLE_INPUT, solve_part1, solve_part2};

// ===== SOLVE FUNCTION TESTS =====

#[test]
fn test_solve_part1_example_input() {
    assert_eq!(solve_part1(EXAMPLE_INPUT).unwrap(), 357);
}

#[test]
fn test_solve_part2_example_input() {
    assert_eq!(solve_part2(EXAMPLE_INPUT, 12).unwrap(), 3_121_910_778_619);
}

#[test]
fn test_solve_part2_clears_stale_suffix_after_replacement() {
    assert_eq!(solve_part2("21800", 3).unwrap(), 800);
}
