use std::ops::RangeInclusive;

/// Solution for day 5, part 1.
#[must_use]
pub fn solve_part1(inventory: &Inventory) -> usize {
    inventory
        .ingredients
        .iter()
        .filter(|ingredient| {
            inventory
                .fresh_ranges
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .count()
}

/// Solution for day 5, part 2.
#[must_use]
pub fn solve_part2(inventory: &Inventory) -> i64 {
    inventory
        .fresh_ranges
        .iter()
        .map(|range| 1 + range.end() - range.start())
        .sum()
}

/// Kitchen inventory.
#[derive(Debug)]
pub struct Inventory {
    fresh_ranges: Vec<RangeInclusive<i64>>,
    ingredients: Vec<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context as _, Result as AnyResult};

    impl Inventory {
        /// Initialize a new inventory.
        ///
        /// Merge overlapping ranges together.
        fn new(
            mut fresh_ranges: Vec<RangeInclusive<i64>>,
            ingredients: Vec<i64>,
        ) -> Self {
            fresh_ranges.sort_unstable_by_key(|range| *range.start());

            let mut merged_ranges = Vec::with_capacity(fresh_ranges.len());
            let mut curr = (*fresh_ranges[0].start(), *fresh_ranges[0].end());
            for range in fresh_ranges {
                if *range.start() <= curr.1 {
                    curr.1 = curr.1.max(*range.end());
                } else {
                    merged_ranges.push(curr.0..=curr.1);
                    curr = range.into_inner();
                }
            }
            merged_ranges.push(curr.0..=curr.1);

            Self {
                fresh_ranges: merged_ranges,
                ingredients,
            }
        }
    }

    fn load_input() -> AnyResult<Inventory> {
        let input = std::fs::read_to_string("input/day05.txt")?;

        let mut lines = input.lines().map(str::trim);

        // First section: ranges (until a blank line).
        let mut fresh_ranges = Vec::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let range = line.split_once('-').context("parse range")?;
            let lo = range.0.parse::<i64>().context("parse lower bound")?;
            let hi = range.1.parse::<i64>().context("parse upper bound")?;

            fresh_ranges.push(lo..=hi);
        }

        // Second section: ingredient IDs
        let mut ingredients = Vec::new();
        for line in lines {
            ingredients.push(line.parse().expect("parse ingredient"));
        }

        Ok(Inventory::new(fresh_ranges, ingredients))
    }

    #[test]
    fn example_part1() {
        let inventory = Inventory::new(
            vec![3..=5, 10..=14, 16..=20, 12..=18],
            vec![1, 5, 8, 11, 17, 32],
        );

        let result = solve_part1(&inventory);

        assert_eq!(result, 3);
    }

    #[test]
    fn real_part1() -> AnyResult<()> {
        let inventory = load_input().context("load input")?;
        let result = solve_part1(&inventory);

        assert_eq!(result, 635);

        Ok(())
    }

    #[test]
    fn example_part2() {
        let inventory = Inventory::new(
            vec![3..=5, 10..=14, 16..=20, 12..=18],
            vec![1, 5, 8, 11, 17, 32],
        );

        let result = solve_part2(&inventory);

        assert_eq!(result, 14);
    }

    #[test]
    fn real_part2() -> AnyResult<()> {
        let inventory = load_input().context("load input")?;
        let result = solve_part2(&inventory);

        assert_eq!(result, 369_761_800_782_619);

        Ok(())
    }
}
