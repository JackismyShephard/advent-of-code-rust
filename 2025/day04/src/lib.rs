//! Day 4
//!
//! Solution scaffold for Advent of Code 2025 4.

use anyhow::{Result, bail};

/// Example input from the problem statement used for testing and
/// documentation.
pub const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

/// Solves Part 1 for the puzzle input.
///
/// # Parameters
/// * `input` - Raw puzzle input text
///
/// # Returns
/// The Part 1 answer once implemented
///
/// # Errors
/// Returns an error until the solution is implemented
pub fn solve_part1(input: &str) -> Result<u64> {
    let mut total_sum = 0;
    let grid = Grid::new(input)?;
    for i in 1..grid.height - 1 {
        for j in 1..grid.width - 1 {
            if grid.get(i, j).value == 1 {
                let mut count = 0;
                for (m, n) in grid.neighbors(i, j) {
                    count += grid.get(m, n).value;
                    if count == 4 {
                        break;
                    }
                }
                if count < 4 {
                    total_sum += 1
                }
            }
        }
    }
    Ok(total_sum)
}

/// Solves Part 2 for the puzzle input.
///
/// # Parameters
/// * `input` - Raw puzzle input text
///
/// # Returns
/// The Part 2 answer once implemented
///
/// # Errors
/// Returns an error until the solution is implemented
pub fn solve_part2(input: &str) -> Result<u64> {
    let mut total_sum = 0;
    let mut grid = Grid::new(input)?;
    let mut stack = Vec::<(usize, usize)>::new();
    for i in 1..grid.height - 1 {
        for j in 1..grid.width - 1 {
            if grid.get(i, j).value == 1 {
                let neighbor_count = grid
                    .neighbors(i, j)
                    .iter()
                    .map(|&(m, n)| grid.get(m, n).value)
                    .sum();
                grid.get_mut(i, j).neighbor_count = neighbor_count;
                if neighbor_count < 4 {
                    stack.push((i, j));
                }
            }
        }
    }
    while let Some((i, j)) = stack.pop() {
        total_sum += 1;
        grid.get_mut(i, j).value = 0;
        for (m, n) in grid.neighbors(i, j) {
            let neighbor = grid.get_mut(m, n);
            if neighbor.value != 1 {
                continue;
            }
            neighbor.neighbor_count -= 1;
            if neighbor.neighbor_count == 3 {
                stack.push((m, n));
            }
        }
    }
    Ok(total_sum)
}
#[derive(Clone, Copy)]
struct Cell {
    value: u8,
    neighbor_count: u8,
}
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}
impl Grid {
    fn new(input: &str) -> Result<Self> {
        let mut cells = Vec::<Cell>::new();
        let mut width = 0;
        let mut height = 0;
        let empty_cell = Cell {
            value: 0,
            neighbor_count: 0,
        };
        for (row, line) in input.lines().enumerate() {
            if row == 0 {
                width = line.len() + 2;
                cells.resize(width, empty_cell);
                height += 1;
            } else if width - 2 != line.len() {
                bail!("Each row must contain the same number of columns")
            }
            height += 1;
            cells.push(empty_cell);
            cells.extend(line.bytes().map(|byte| Cell {
                value: u8::from(byte == b'@'),
                neighbor_count: 0,
            }));
            cells.push(empty_cell);
        }
        cells.resize(cells.len() + width, empty_cell);
        height += 1;

        Ok(Self {
            width,
            height,
            cells,
        })
    }

    fn get(&self, i: usize, j: usize) -> Cell {
        self.cells[i * self.width + j]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut Cell {
        &mut self.cells[i * self.width + j]
    }

    fn neighbors(&self, i: usize, j: usize) -> [(usize, usize); 8] {
        [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ]
    }
}
