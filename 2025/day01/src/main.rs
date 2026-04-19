use anyhow::Result;
use y2025_d01::{
    solve_part1, solve_part2, solve_part2_unwrapped_intervals, solve_part2_wrapped_intervals,
    EXAMPLE_INPUT,
};

fn main() -> Result<()> {
    println!("=== Day 1 ===");
    println!();

    println!("=== Example Input Results ===");
    let result1 = solve_part1(EXAMPLE_INPUT)?;
    println!("Part 1 example result: {result1}");

    let result2 = solve_part2(EXAMPLE_INPUT)?;
    println!("Part 2 example result: {result2}");

    let wrapped_result2 = solve_part2_wrapped_intervals(EXAMPLE_INPUT)?;
    println!("Part 2 wrapped intervals example result: {wrapped_result2}");

    let unwrapped_result2 = solve_part2_unwrapped_intervals(EXAMPLE_INPUT)?;
    println!("Part 2 unwrapped intervals example result: {unwrapped_result2}");

    if let Ok(input) = shared::read_local_input!() {
        println!("\n=== Real Input Results ===");
        let result1 = solve_part1(&input)?;
        println!("Part 1 result: {result1}");

        let result2 = solve_part2(&input)?;
        println!("Part 2 result: {result2}");

        let wrapped_result2 = solve_part2_wrapped_intervals(&input)?;
        println!("Part 2 wrapped intervals result: {wrapped_result2}");

        let unwrapped_result2 = solve_part2_unwrapped_intervals(&input)?;
        println!("Part 2 unwrapped intervals result: {unwrapped_result2}");
    } else {
        println!("No input.txt found - create input.txt in this day crate with your puzzle input");
    }

    Ok(())
}
