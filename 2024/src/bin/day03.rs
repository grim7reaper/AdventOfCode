use anyhow::{Context as _, Result as AnyResult};
use regex::Regex;

fn main() -> AnyResult<()> {
    let memory = load_input().context("load input")?;

    let result = solve_part1(&memory);
    println!("Part 1: {result}");
    assert_eq!(result, 165225049);

    let result = solve_part2(&memory);
    println!("Part 2: {result}");
    assert_eq!(result, 108830766);

    Ok(())
}

fn solve_part1(memory: &str) -> i32 {
    let instructions_re = Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)")
        .expect("instructions regex");

    instructions_re
        .captures_iter(memory)
        .map(|captures| {
            let lhs = &captures["lhs"].parse::<i32>().expect("lhs");
            let rhs = &captures["rhs"].parse::<i32>().expect("lhs");
            lhs * rhs
        })
        .sum()
}

fn solve_part2(memory: &str) -> i32 {
    let instructions_re = Regex::new(
        r"(?:mul\((?<lhs>\d+),(?<rhs>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .expect("instructions regex");

    instructions_re
        .captures_iter(memory)
        .fold((0, true), |(sum, enabled), captures| {
            if captures.name("do").is_some() {
                (sum, true)
            } else if captures.name("dont").is_some() {
                (sum, false)
            } else if enabled {
                let lhs = &captures["lhs"].parse::<i32>().expect("lhs");
                let rhs = &captures["rhs"].parse::<i32>().expect("lhs");
                (sum + lhs * rhs, enabled)
            } else {
                (sum, enabled)
            }
        })
        .0
}

fn load_input() -> AnyResult<String> {
    std::fs::read_to_string("input/day03.txt").context("read input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = solve_part1(memory);

        assert_eq!(result, 161);
    }

    #[test]
    fn part2() {
        let memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = solve_part2(memory);

        assert_eq!(result, 48);
    }
}
