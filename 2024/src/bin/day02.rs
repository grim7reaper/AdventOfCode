use anyhow::{Context as _, Result as AnyResult};
use rayon::prelude::*;
use std::{fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let reports = load_input().context("load input")?;

    let result = solve_part1(&reports);
    println!("Part 1: {result}");
    assert_eq!(result, 421);

    let result = solve_part2(&reports);
    println!("Part 2: {result}");
    assert_eq!(result, 476);

    Ok(())
}

fn solve_part1(reports: &[Vec<i32>]) -> usize {
    reports
        .par_iter()
        .filter(|report| report_is_safe(report))
        .count()
}

fn solve_part2(reports: &[Vec<i32>]) -> usize {
    reports
        .par_iter()
        .filter(|report| {
            match report.len() {
                // a one-level report is always safe: no diff, no order
                1 => true,
                // a two-level report is always safe: can delete one if needed.
                2 => true,
                3 => {
                    if report_is_safe(report) {
                        return true;
                    }
                    for i in 0..3 {
                        let mut fixed = report.to_vec();
                        fixed.remove(i);
                        if report_is_safe(&fixed) {
                            return true;
                        }
                    }
                    false
                }
                // We now have enough data to compute the "direction".
                _ => {
                    let mut signums = [
                        (report[0] - report[1]).signum(),
                        (report[1] - report[2]).signum(),
                        (report[2] - report[3]).signum(),
                    ];
                    signums.sort_unstable();
                    let sign = signums[1];

                    let mut shit_index = Vec::with_capacity(2);
                    for i in 0..report.len() - 1 {
                        let a = report[i];
                        let b = report[i + 1];
                        let diff = a - b;
                        if !(1..=3).contains(&diff.abs())
                            || diff.signum() != sign
                        {
                            shit_index.push(i);
                            shit_index.push(i + 1);
                            break;
                        }
                    }
                    let mut ko = 0;
                    for bad in shit_index {
                        for i in 0..report.len() - 1 {
                            if i == bad {
                                continue;
                            }
                            let j = if i + 1 == bad { i + 2 } else { i + 1 };
                            if j >= report.len() {
                                continue;
                            }
                            let a = report[i];
                            let b = report[j];
                            let diff = a - b;
                            if !(1..=3).contains(&diff.abs())
                                || diff.signum() != sign
                            {
                                ko += 1;
                                break;
                            }
                        }
                    }
                    ko < 2
                }
            }
        })
        .count()
}

fn report_is_safe(report: &[i32]) -> bool {
    if report.len() >= 2 {
        let sign = (report[0] - report[1]).signum();
        for pair in report.windows(2) {
            let diff = pair[0] - pair[1];
            if !(1..=3).contains(&diff.abs()) || diff.signum() != sign {
                return false;
            }
        }
    }

    true
}

fn load_input() -> AnyResult<Vec<Vec<i32>>> {
    let file = File::open("input/day02.txt").context("open input file")?;

    std::io::BufReader::new(file).lines().try_fold(
        Vec::new(),
        |mut acc, line| {
            let line = line.context("read input line")?;
            let levels = line
                .split(' ')
                .map(|value| value.parse::<i32>().context("invalid value"))
                .collect::<AnyResult<Vec<_>>>()?;
            acc.push(levels);
            Ok(acc)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let result = solve_part1(&reports);

        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let result = solve_part2(&reports);

        assert_eq!(result, 4);
    }
}
