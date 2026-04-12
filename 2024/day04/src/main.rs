use anyhow::Result;
use y2024_d04::{solve_part1, solve_part2, EXAMPLE_INPUT};

fn main() -> Result<()> {
    println!("=== Day 4: Ceres Search ===");
    println!();

    // Test with example input
    println!("=== Example Input Results ===");
    let example_result_part1 = solve_part1(EXAMPLE_INPUT);
    println!("Part 1 example result: {example_result_part1}");

    let example_result_part2 = solve_part2(EXAMPLE_INPUT);
    println!("Part 2 example result: {example_result_part2}");
    println!();

    // Try to read actual input file if it exists
    if let Ok(input) = shared::read_local_input!() {
        println!("=== Real Input Results ===");
        let part1_result = solve_part1(&input);
        println!("Part 1 result: {part1_result}");
        let part2_result = solve_part2(&input);
        println!("Part 2 result: {part2_result}");
    } else {
        println!("No input.txt found - create input.txt in this day crate with your puzzle input");
    }

    Ok(())
}
