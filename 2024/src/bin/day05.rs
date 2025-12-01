use anyhow::{Context as _, Result as AnyResult};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::prelude::*,
};

fn main() -> AnyResult<()> {
    let input = load_input().context("load input")?;

    let result = solve_part1(&input);
    println!("Part 1: {result}");
    assert_eq!(result, 5762);

    let result = solve_part2(&input);
    println!("Part 2: {result}");
    assert_eq!(result, 4130);

    Ok(())
}

fn solve_part1(input: &Input) -> i32 {
    input
        .updates
        .iter()
        .filter(|update| {
            let all_pages = update.iter().copied().collect::<HashSet<_>>();
            let mut printed = HashSet::new();
            for page in *update {
                if let Some(deps) = input.rules.get(page) {
                    let relevant_rules = deps & &all_pages;
                    let missing = relevant_rules.difference(&printed).next();
                    if missing.is_some() {
                        return false;
                    }
                }
                printed.insert(*page);
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve_part2(input: &Input) -> i32 {
    input
        .updates
        .iter()
        .filter(|update| {
            let all_pages = update.iter().copied().collect::<HashSet<_>>();
            let mut printed = HashSet::new();
            for page in *update {
                if let Some(deps) = input.rules.get(page) {
                    let relevant_rules = deps & &all_pages;
                    let missing = relevant_rules.difference(&printed).next();
                    if missing.is_some() {
                        return true;
                    }
                }
                printed.insert(*page);
            }
            false
        })
        .map(|update| {
            let mut fixed = update.to_vec();
            fixed.sort_unstable_by(|a, b| {
                let a_dep_b = input
                    .rules
                    .get(a)
                    .map(|deps| deps.contains(b))
                    .unwrap_or_default();
                if a_dep_b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            fixed
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[derive(Debug)]
struct Input {
    rules: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

fn load_input() -> AnyResult<Input> {
    let file = File::open("input/day05.txt").context("open input file")?;
    let mut lines = std::io::BufReader::new(file).lines();

    // Parse rules.
    let mut rules: HashMap<i32, HashSet<_>> = HashMap::new();
    for line in &mut lines {
        let line = line.context("read rule line")?;
        if line.is_empty() {
            break;
        }
        let parts = line.split_once('|').context("parse rule")?;
        let deps = parts.0.parse::<i32>().context("parse dependency")?;
        let page = parts.1.parse::<i32>().context("parse page number")?;
        rules.entry(page).or_default().insert(deps);
    }

    // Parse updates.
    let mut updates = Vec::new();
    for line in lines {
        let line = line.context("read update line")?;
        updates.push(
            line.split(',')
                .map(|value| value.parse::<i32>().context("update page number"))
                .collect::<AnyResult<Vec<_>>>()?,
        );
    }

    Ok(Input { rules, updates })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let rules = [
            (53i32, vec![47i32, 97, 61, 75]),
            (47, vec![97, 75]),
            (75, vec![97]),
            (13, vec![29, 75, 97, 53, 61, 47]),
            (29, vec![53, 47, 97, 61, 75]),
            (61, vec![75, 47, 97]),
        ]
        .iter()
        .map(|(page, deps)| {
            (*page, deps.iter().copied().collect::<HashSet<_>>())
        })
        .collect::<HashMap<_, _>>();
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let input = Input { rules, updates };
        let result = solve_part1(&input);

        assert_eq!(result, 143);
    }

    #[test]
    fn part2() {
        let rules = [
            (53i32, vec![47i32, 97, 61, 75]),
            (47, vec![97, 75]),
            (75, vec![97]),
            (13, vec![29, 75, 97, 53, 61, 47]),
            (29, vec![53, 47, 97, 61, 75]),
            (61, vec![75, 47, 97]),
        ]
        .iter()
        .map(|(page, deps)| {
            (*page, deps.iter().copied().collect::<HashSet<_>>())
        })
        .collect::<HashMap<_, _>>();
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let input = Input { rules, updates };
        let result = solve_part2(&input);

        assert_eq!(result, 123);
    }
}
