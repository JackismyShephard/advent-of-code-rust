use anyhow::{bail, Context, Result};

pub const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

pub fn solve_part1(input: &str) -> Result<u32> {
    let mut code = 0;
    let mut start = 50;
    for line in input.lines() {
        let step = parse_step(line)?;
        start = (start + step).rem_euclid(100);
        if start == 0 {
            code += 1;
        }
    }
    Ok(code)
}

pub fn solve_part2(input: &str) -> Result<u32> {
    let mut code = 0;
    let mut start: i32 = 50;
    for line in input.lines() {
        let step = parse_step(line)?;
        // dial touches 0 once for each full turn
        code += step.unsigned_abs() / 100;
        let remainder_end = start + step % 100;
        // dial touches 0 once more if position after remaining steps is greater than 100
        // or less than or equal to 0 when start position is not 0
        if remainder_end >= 100 || start != 0 && remainder_end <= 0 {
            code += 1
        }
        start = (start + step).rem_euclid(100);
    }
    Ok(code)
}

pub fn solve_part2_wrapped_intervals(input: &str) -> Result<u32> {
    let mut code = 0;
    let mut position = 50;
    for line in input.lines() {
        let step = parse_step(line)?;
        let end = position + step;
        if step.is_positive() {
            code += end / 100;
        } else if step.is_negative() {
            let start = if position == 0 { -1 } else { 0 };
            code += start - (end - 1).div_euclid(100);
        }
        position = end.rem_euclid(100);
    }
    Ok(code as u32)
}

pub fn solve_part2_unwrapped_intervals(input: &str) -> Result<u64> {
    let mut code = 0;
    let mut position = 50_i64;
    for line in input.lines() {
        let step = i64::from(parse_step(line)?);
        let end = position + step;
        if step.is_positive() {
            code += end.div_euclid(100) - position.div_euclid(100);
        } else if step.is_negative() {
            code += (position - 1).div_euclid(100) - (end - 1).div_euclid(100);
        }
        position = end;
    }
    Ok(code as u64)
}

fn parse_step(line: &str) -> Result<i32> {
    let (direction, amount_str) = line.split_at_checked(1).with_context(|| {
        format!("line must contain at least one character indicating rotation direction: {line:?}")
    })?;
    let amount: i32 = amount_str
        .parse()
        .with_context(|| format!("line must contain a valid rotation amount: {line:?}"))?;
    let step = match direction {
        "R" => amount,
        "L" => -amount,
        _ => bail!("rotation direction must be either L or R, got {direction:?}"),
    };

    Ok(step)
}
