use anyhow::{Context as _, Result as AnyResult};
use std::{collections::HashMap, fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let (list1, list2) = load_input().context("load input")?;

    let result = solve_part1(list1.clone(), list2.clone());
    println!("Part 1: {result}");
    assert_eq!(result, 3569916);

    let result = solve_part2(&list1, &list2);
    println!("Part 2: {result}");
    assert_eq!(result, 26407426);

    Ok(())
}

fn solve_part1(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn solve_part2(list1: &[i32], list2: &[i32]) -> i32 {
    let counters = list2.iter().fold(HashMap::new(), |mut acc, value| {
        *acc.entry(value).or_default() += 1;
        acc
    });

    list1
        .iter()
        .copied()
        .map(|value| value * counters.get(&value).unwrap_or(&0))
        .sum()
}

fn load_input() -> AnyResult<(Vec<i32>, Vec<i32>)> {
    let file = File::open("input/day01.txt").context("open input file")?;

    std::io::BufReader::new(file).lines().try_fold(
        (Vec::new(), Vec::new()),
        |(mut list1, mut list2), line| {
            let line = line.context("read input line")?;
            let (value1, value2) =
                line.split_once("   ").context("bad line")?;

            list1.push(value1.parse::<i32>().context("invalid first value")?);
            list2.push(value2.parse::<i32>().context("invalid second value")?);

            Ok((list1, list2))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let result = solve_part1(list1, list2);

        assert_eq!(result, 11);
    }

    #[test]
    fn part2() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let result = solve_part2(&list1, &list2);

        assert_eq!(result, 31);
    }
}
