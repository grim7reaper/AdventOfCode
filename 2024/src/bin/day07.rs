use anyhow::{Context as _, Result as AnyResult};
use rayon::prelude::*;
use std::{collections::VecDeque, fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let equations = load_input().context("load input")?;

    let result = solve_part1(&equations);
    println!("Part 1: {result}");
    assert_eq!(result, 1582598718861);

    let result = solve_part2(&equations);
    println!("Part 2: {result}");
    assert_eq!(result, 165278151522644);

    Ok(())
}

fn solve_part1(equations: &[Equation]) -> u64 {
    equations
        .par_iter()
        .filter(|equation| {
            let k = equation.operands.len() - 1;
            let mut queue = VecDeque::new();
            queue.push_back(vec![Operator::Mul]);
            queue.push_back(vec![Operator::Add]);
            while !queue.is_empty() {
                let operators = queue.pop_front().expect("operators");
                match equation.evaluate(&operators).cmp(&equation.result) {
                    // Too big already, stop here.
                    std::cmp::Ordering::Greater => continue,
                    std::cmp::Ordering::Equal => {
                        if operators.len() == k {
                            return true;
                        }
                    }
                    std::cmp::Ordering::Less => {
                        // Max depth reached, stop here.
                        if operators.len() == k {
                            continue;
                        }
                    }
                }
                for op in [Operator::Mul, Operator::Add] {
                    let mut new = operators.clone();
                    new.push(op);
                    queue.push_back(new);
                }
            }
            false
        })
        .map(|equation| equation.result)
        .sum()
}

fn solve_part2(equations: &[Equation]) -> u64 {
    equations
        .par_iter()
        .filter(|equation| {
            let k = equation.operands.len() - 1;
            let mut queue = VecDeque::new();
            queue.push_back(vec![Operator::Mul]);
            queue.push_back(vec![Operator::Concat]);
            queue.push_back(vec![Operator::Add]);
            while !queue.is_empty() {
                let operators = queue.pop_front().expect("operators");
                match equation.evaluate(&operators).cmp(&equation.result) {
                    // Too big already, stop here.
                    std::cmp::Ordering::Greater => continue,
                    std::cmp::Ordering::Equal => {
                        if operators.len() == k {
                            return true;
                        }
                    }
                    std::cmp::Ordering::Less => {
                        // Max depth reached, stop here.
                        if operators.len() == k {
                            continue;
                        }
                    }
                }
                for op in [Operator::Mul, Operator::Concat, Operator::Add] {
                    let mut new = operators.clone();
                    new.push(op);
                    queue.push_back(new);
                }
            }
            false
        })
        .map(|equation| equation.result)
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn evaluate(&self, operators: &[Operator]) -> u64 {
        operators.iter().zip(&self.operands[1..]).fold(
            self.operands[0],
            |acc, (op, value)| match op {
                Operator::Add => acc + value,
                Operator::Mul => acc * value,
                Operator::Concat => {
                    let digits = value.ilog10() + 1;
                    acc * 10u64.pow(digits) + value
                }
            },
        )
    }
}

fn load_input() -> AnyResult<Vec<Equation>> {
    let file = File::open("input/day07.txt").context("open input file")?;
    std::io::BufReader::new(file).lines().try_fold(
        Vec::new(),
        |mut acc, line| {
            let line = line.context("read line")?;
            let (left, right) =
                line.split_once(": ").context("parse equation")?;
            let result = left.parse::<u64>().context("parse result")?;
            let operands = right
                .split(' ')
                .map(|value| value.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .context("parse operands")?;
            acc.push(Equation { result, operands });
            Ok(acc)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        #[rustfmt::skip]
        let equations = vec![
            Equation { result: 190,    operands: vec![10, 19] },
            Equation { result: 3267,   operands: vec![81, 40, 27] },
            Equation { result: 83,     operands: vec![17, 5] },
            Equation { result: 156,    operands: vec![15, 6] },
            Equation { result: 7290,   operands: vec![6, 8, 6, 15] },
            Equation { result: 161011, operands: vec![16, 10, 13] },
            Equation { result: 192,    operands: vec![17, 8, 14] },
            Equation { result: 21037,  operands: vec![9, 7, 18, 13] },
            Equation { result: 292,    operands: vec![11, 6, 16, 20] },
        ];
        let result = solve_part1(&equations);

        assert_eq!(result, 3749);
    }

    #[test]
    fn part2() {
        #[rustfmt::skip]
        let equations = vec![
            Equation { result: 190,    operands: vec![10, 19] },
            Equation { result: 3267,   operands: vec![81, 40, 27] },
            Equation { result: 83,     operands: vec![17, 5] },
            Equation { result: 156,    operands: vec![15, 6] },
            Equation { result: 7290,   operands: vec![6, 8, 6, 15] },
            Equation { result: 161011, operands: vec![16, 10, 13] },
            Equation { result: 192,    operands: vec![17, 8, 14] },
            Equation { result: 21037,  operands: vec![9, 7, 18, 13] },
            Equation { result: 292,    operands: vec![11, 6, 16, 20] },
        ];
        let result = solve_part2(&equations);

        assert_eq!(result, 11387);
    }
}
