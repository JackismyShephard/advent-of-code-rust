use anyhow::Result;
use y2025_d02::{solve_part1, solve_part2, EXAMPLE_INPUT};

fn main() -> Result<()> {
    println!("=== Day 2 ===");
    println!();

    println!("=== Example Input Results ===");
    match solve_part1(EXAMPLE_INPUT) {
        Ok(result) => println!("Part 1 example result: {result}"),
        Err(error) => println!("Part 1 example error: {error}"),
    }

    match solve_part2(EXAMPLE_INPUT) {
        Ok(result) => println!("Part 2 example result: {result}"),
        Err(error) => println!("Part 2 example error: {error}"),
    }
    println!();

    if let Ok(input) = shared::read_local_input!() {
        println!("=== Real Input Results ===");

        match solve_part1(&input) {
            Ok(result) => println!("Part 1 result: {result}"),
            Err(error) => println!("Part 1 error: {error}"),
        }

        match solve_part2(&input) {
            Ok(result) => println!("Part 2 result: {result}"),
            Err(error) => println!("Part 2 error: {error}"),
        }
    } else {
        println!("No input.txt found - create input.txt in this day crate with your puzzle input");
    }

    Ok(())
}
