/// Solution for day 3, part 1.
#[must_use]
pub fn solve_part1(banks: &[String]) -> usize {
    banks
        .iter()
        .map(|bank| {
            // Bank only contains ASCII digit, easier to treat it as bytes.
            let digits = bank.as_bytes();
            let (i, a) = pick_first_max(&digits[..bank.len() - 1]);
            let (_, b) = pick_first_max(&digits[i + 1..]);
            usize::from(a - b'0') * 10 + usize::from(b - b'0')
        })
        .sum()
}

/// Solution for day 3, part 2.
#[must_use]
pub fn solve_part2(banks: &[String]) -> usize {
    banks
        .iter()
        .map(|bank| {
            // Bank only contains ASCII digit, easier to treat it as bytes.
            let digits = bank.as_bytes();

            let mut res = 0;
            let mut offset = 0;
            for n in (0..12_u8).rev() {
                let end = bank.len() - usize::from(n);
                let (i, v) = pick_first_max(&digits[offset..end]);
                res += usize::from(v - b'0') * 10_usize.pow(n.into());
                offset += i + 1;
            }

            res
        })
        .sum()
}

fn pick_first_max(bytes: &[u8]) -> (usize, u8) {
    bytes
        .iter()
        .enumerate()
        .max_by_key(|(idx, value)| (*value, std::cmp::Reverse(*idx)))
        .map(|(idx, value)| (idx, *value))
        .expect("slice is non-empty")
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context as _, Result as AnyResult};
    use std::{fs::File, io::prelude::*};

    fn load_input() -> AnyResult<Vec<String>> {
        let file = File::open("input/day03.txt").context("open input file")?;

        std::io::BufReader::new(file).lines().try_fold(
            Vec::new(),
            |mut acc, line| {
                let line = line.context("read input line")?;
                acc.push(line.trim_end().to_owned());
                Ok(acc)
            },
        )
    }

    #[test]
    fn example_part1() {
        let banks = vec![
            "987654321111111".to_owned(),
            "811111111111119".to_owned(),
            "234234234234278".to_owned(),
            "818181911112111".to_owned(),
        ];
        let result = solve_part1(&banks);

        assert_eq!(result, 357);
    }

    #[test]
    fn real_part1() -> AnyResult<()> {
        let banks = load_input().context("load input")?;
        let result = solve_part1(&banks);

        assert_eq!(result, 17_435);

        Ok(())
    }

    #[test]
    fn example_part2() {
        let banks = vec![
            "987654321111111".to_owned(),
            "811111111111119".to_owned(),
            "234234234234278".to_owned(),
            "818181911112111".to_owned(),
        ];
        let result = solve_part2(&banks);

        assert_eq!(result, 3_121_910_778_619);
    }

    #[test]
    fn real_part2() -> AnyResult<()> {
        let banks = load_input().context("load input")?;
        let result = solve_part2(&banks);

        assert_eq!(result, 172_886_048_065_379);

        Ok(())
    }
}
