use anyhow::{Context as _, Result as AnyResult};
use std::collections::HashMap;

fn main() -> AnyResult<()> {
    let stones = load_input().context("load input")?;

    let result = solve_part1(&stones);
    println!("Part 1: {result}");
    assert_eq!(result, 186996);

    let result = solve_part2(&stones);
    println!("Part 2: {result}");
    assert_eq!(result, 221683913164898);

    Ok(())
}

fn solve_part1(stones: &[u64]) -> usize {
    count_stones(stones, 25)
}

fn solve_part2(stones: &[u64]) -> usize {
    count_stones(stones, 75)
}

fn count_stones(stones: &[u64], blink: usize) -> usize {
    let mut counters =
        stones.iter().fold(HashMap::new(), |mut counters, stone| {
            *counters.entry(*stone).or_default() += 1;
            counters
        });

    let mut next_gen = HashMap::new();
    for _ in 0..blink {
        for (stone, count) in &counters {
            if *stone == 0 {
                *next_gen.entry(1).or_default() += count;
            } else if let Some((lo, hi)) = split(*stone) {
                *next_gen.entry(lo).or_default() += count;
                *next_gen.entry(hi).or_default() += count;
            } else {
                *next_gen.entry(*stone * 2024).or_default() += count;
            }
        }
        std::mem::swap(&mut counters, &mut next_gen);
        next_gen.clear();
    }
    counters.values().sum()
}

fn split(value: u64) -> Option<(u64, u64)> {
    let digits = value.ilog10() + 1;
    if digits & 1 == 1 {
        return None;
    }
    let pow = 10u64.pow(digits >> 1);
    Some((value / pow, value % pow))
}

fn load_input() -> AnyResult<Vec<u64>> {
    let input =
        std::fs::read_to_string("input/day11.txt").context("read input")?;
    input
        .trim_end()
        .split(' ')
        .map(|value| value.parse::<u64>().context("invalid number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let stones = [125, 17];
        let result = solve_part1(&stones);

        assert_eq!(result, 55312);
    }

    #[test]
    fn part2() {
        let stones = [125, 17];
        let result = solve_part2(&stones);

        assert_eq!(result, 65601038650482);
    }
}
