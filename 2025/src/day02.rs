use std::ops::RangeInclusive;

/// Solution for day 2, part 1.
#[must_use]
#[expect(clippy::string_slice, reason = "numbers as string are ASCII")]
pub fn solve_part1(ranges: &[RangeInclusive<i64>]) -> i64 {
    ranges
        .iter()
        .cloned()
        .flatten()
        .filter(|id| {
            let digits = id.to_string();
            let mid = digits.len() / 2;
            digits[..mid] == digits[mid..]
        })
        .sum()
}

/// Solution for day 2, part 2.
#[must_use]
#[expect(clippy::missing_panics_doc, reason = "don't care here")]
pub fn solve_part2(mut ranges: Vec<RangeInclusive<i64>>) -> i64 {
    // XXX: Assuming range don't overlap!
    ranges.sort_unstable_by_key(|range| *range.start());
    let max_value = *ranges.last().expect("non-empty input").end();

    // Instead of checking every number in the range, generate ALL
    // repeated-pattern numbers and intersect with the range (way less number to
    // check!)
    let mut vals = invalid_ids(max_value)
        .filter(|&id| {
            match ranges.binary_search_by_key(&id, |range| *range.start()) {
                Ok(_) => true,
                Err(idx) => {
                    // Smaller than our smallest range
                    if idx == 0 {
                        return false;
                    }
                    // Check if it's in range.
                    id <= *ranges[idx - 1].end()
                }
            }
        })
        .collect::<Vec<_>>();

    // `invalid_ids` returns duplicates, so filter before summing the values.
    vals.sort_unstable();
    vals.dedup();

    vals.iter().sum()
}

fn invalid_ids(max_value: i64) -> impl Iterator<Item = i64> {
    let max_width = max_value.checked_ilog10().unwrap_or(0) + 1;

    (2..=max_width).flat_map(move |width| {
        (1..=width / 2)
            .filter(move |&blk_width| width.is_multiple_of(blk_width))
            .flat_map(move |blk_width| {
                let repeat_count = width / blk_width;
                let start = 10_i64.pow(blk_width - 1);
                let end = 10_i64.pow(blk_width);

                (start..end)
                    .map(move |block| {
                        repeat_block(block, repeat_count, blk_width)
                    })
                    .take_while(move |&value| value <= max_value)
            })
    })
}

fn repeat_block(block: i64, repeat_count: u32, blk_width: u32) -> i64 {
    let mul = 10_i64.pow(blk_width);
    let mut n = 0_i64;
    for _ in 0..repeat_count {
        n = n * mul + block;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context as _, Result as AnyResult};

    fn load_input() -> AnyResult<Vec<RangeInclusive<i64>>> {
        std::fs::read_to_string("input/day02.txt")
            .context("read input")?
            .trim_end()
            .split(',')
            .map(|value| {
                let range = value.split_once('-').context("parse range")?;
                let lo = range.0.parse::<i64>().context("parse lower bound")?;
                let hi = range.1.parse::<i64>().context("parse upper bound")?;

                Ok(lo..=hi)
            })
            .collect::<AnyResult<_>>()
    }

    #[test]
    fn example_part1() {
        let ranges = vec![
            11..=22,
            95..=115,
            998..=1012,
            1_188_511_880..=1_188_511_890,
            222_220..=222_224,
            1_698_522..=1_698_528,
            446_443..=446_449,
            38_593_856..=38_593_862,
            565_653..=565_659,
            824_824_821..=824_824_827,
            2_121_212_118..=2_121_212_124,
        ];
        let result = solve_part1(&ranges);

        assert_eq!(result, 1_227_775_554);
    }

    #[test]
    fn real_part1() -> AnyResult<()> {
        let ranges = load_input().context("load input")?;
        let result = solve_part1(&ranges);

        assert_eq!(result, 23_039_913_998);

        Ok(())
    }

    #[test]
    fn repeat_blk() {
        assert_eq!(repeat_block(1, 3, 1), 111);
        assert_eq!(repeat_block(11885, 2, 5), 1_188_511_885);
        assert_eq!(repeat_block(56, 3, 2), 565_656);
        assert_eq!(repeat_block(21, 5, 2), 2_121_212_121);
    }

    #[test]
    fn example_part2() {
        let ranges = vec![
            11..=22,
            95..=115,
            998..=1012,
            1_188_511_880..=1_188_511_890,
            222_220..=222_224,
            1_698_522..=1_698_528,
            446_443..=446_449,
            38_593_856..=38_593_862,
            565_653..=565_659,
            824_824_821..=824_824_827,
            2_121_212_118..=2_121_212_124,
        ];
        let result = solve_part2(ranges);

        assert_eq!(result, 4_174_379_265);
    }

    #[test]
    fn real_part2() -> AnyResult<()> {
        let ranges = load_input().context("load input")?;
        let result = solve_part2(ranges);

        assert_eq!(result, 35_950_619_148);

        Ok(())
    }
}
