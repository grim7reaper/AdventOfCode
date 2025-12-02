//! Solution for day 1.
use anyhow::{Context as _, Result as AnyResult, bail, ensure};
use std::{fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let rotations = load_input().context("load input")?;

    let result = solve_part1(&rotations);
    println!("Part 1: {result}");
    assert_eq!(result, 962);

    let result = solve_part2(&rotations);
    println!("Part 2: {result}");
    assert_eq!(result, 5782);

    Ok(())
}

fn solve_part1(rotations: &[i32]) -> usize {
    rotations
        .iter()
        .scan(50, |state, distance| {
            *state = (*state + distance).rem_euclid(100);
            Some(*state)
        })
        .filter(|state| *state == 0)
        .count()
}

fn solve_part2(rotations: &[i32]) -> i32 {
    rotations
        .iter()
        .scan(50, |state, distance| {
            // Count the number of complete turns.
            let mut hits = distance.abs() / 100;

            // Now check the leftover rotation.
            let distance = distance % 100;
            let start = *state;
            *state += distance;

            if *state <= 0 {
                // Ignore case where we start at 0 because that's a full
                // rotation and already accounted for above.
                hits += i32::from(start != 0);
                // Only adjust if necessary.
                *state += 100 * i32::from(*state < 0);
            } else if *state >= 100 {
                hits += 1;
                *state -= 100;
            }

            Some(hits)
        })
        .sum()
}

fn load_input() -> AnyResult<Vec<i32>> {
    let file = File::open("input/day01.txt").context("open input file")?;

    std::io::BufReader::new(file).lines().try_fold(
        Vec::new(),
        |mut acc, line| {
            let line = line.context("read input line")?;
            let (direction, distance) = line.split_at(1);
            let distance =
                distance.parse::<i32>().context("invalid distance")?;
            ensure!(distance >= 0, "negative distance: {distance}");

            match direction {
                "L" => acc.push(-distance),
                "R" => acc.push(distance),
                _ => bail!("invalid direction: {direction}"),
            }

            Ok(acc)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let rotations = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        let result = solve_part1(&rotations);

        assert_eq!(result, 3);
    }

    #[test]
    fn part2() {
        let rotations = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        let result = solve_part2(&rotations);

        assert_eq!(result, 6);
    }
}
