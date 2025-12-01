use anyhow::{Context as _, Result as AnyResult};
use std::{fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let matrix = load_input().context("load input")?;

    let result = solve_part1(&matrix);
    println!("Part 1: {result}");
    assert_eq!(result, 2633);

    let result = solve_part2(&matrix);
    println!("Part 2: {result}");
    assert_eq!(result, 1936);

    Ok(())
}

fn solve_part1(matrix: &[Vec<char>]) -> i32 {
    let h = matrix.len();
    let w = matrix[0].len();

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if matrix[i][j] == 'X' {
                count += search_horizontal(matrix, i, j, w);
                count += search_vertical(matrix, i, j, h);
                count += search_diagonal(matrix, i, j, h, w);
            }
        }
    }

    count
}

fn search_horizontal(
    matrix: &[Vec<char>],
    i: usize,
    j: usize,
    w: usize,
) -> i32 {
    let mut count = 0;

    if j + 3 < w {
        count += i32::from(
            matrix[i][j + 1] == 'M'
                && matrix[i][j + 2] == 'A'
                && matrix[i][j + 3] == 'S',
        );
    }
    if j >= 3 {
        count += i32::from(
            matrix[i][j - 1] == 'M'
                && matrix[i][j - 2] == 'A'
                && matrix[i][j - 3] == 'S',
        );
    }

    count
}

fn search_vertical(matrix: &[Vec<char>], i: usize, j: usize, h: usize) -> i32 {
    let mut count = 0;

    if i + 3 < h {
        count += i32::from(
            matrix[i + 1][j] == 'M'
                && matrix[i + 2][j] == 'A'
                && matrix[i + 3][j] == 'S',
        );
    }
    if i >= 3 {
        count += i32::from(
            matrix[i - 1][j] == 'M'
                && matrix[i - 2][j] == 'A'
                && matrix[i - 3][j] == 'S',
        );
    }

    count
}

fn search_diagonal(
    matrix: &[Vec<char>],
    i: usize,
    j: usize,
    h: usize,
    w: usize,
) -> i32 {
    let mut count = 0;

    if j + 3 < w {
        if i + 3 < h {
            count += i32::from(
                matrix[i + 1][j + 1] == 'M'
                    && matrix[i + 2][j + 2] == 'A'
                    && matrix[i + 3][j + 3] == 'S',
            );
        }
        if i >= 3 {
            count += i32::from(
                matrix[i - 1][j + 1] == 'M'
                    && matrix[i - 2][j + 2] == 'A'
                    && matrix[i - 3][j + 3] == 'S',
            );
        }
    }
    if j >= 3 {
        if i + 3 < h {
            count += i32::from(
                matrix[i + 1][j - 1] == 'M'
                    && matrix[i + 2][j - 2] == 'A'
                    && matrix[i + 3][j - 3] == 'S',
            );
        }
        if i >= 3 {
            count += i32::from(
                matrix[i - 1][j - 1] == 'M'
                    && matrix[i - 2][j - 2] == 'A'
                    && matrix[i - 3][j - 3] == 'S',
            );
        }
    }

    count
}

fn solve_part2(matrix: &[Vec<char>]) -> i32 {
    let h = matrix.len();
    let w = matrix[0].len();

    let mut count = 0;
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if matrix[i][j] == 'A' {
                // M.S
                // .A.
                // M.S
                count += i32::from(
                    matrix[i - 1][j - 1] == 'M'
                        && matrix[i - 1][j + 1] == 'S'
                        && matrix[i + 1][j - 1] == 'M'
                        && matrix[i + 1][j + 1] == 'S',
                );
                // M.M
                // .A.
                // S.S
                count += i32::from(
                    matrix[i - 1][j - 1] == 'M'
                        && matrix[i - 1][j + 1] == 'M'
                        && matrix[i + 1][j - 1] == 'S'
                        && matrix[i + 1][j + 1] == 'S',
                );
                // S.M
                // .A.
                // S.M
                count += i32::from(
                    matrix[i - 1][j - 1] == 'S'
                        && matrix[i - 1][j + 1] == 'M'
                        && matrix[i + 1][j - 1] == 'S'
                        && matrix[i + 1][j + 1] == 'M',
                );
                // S.S
                // .A.
                // M.M
                count += i32::from(
                    matrix[i - 1][j - 1] == 'S'
                        && matrix[i - 1][j + 1] == 'S'
                        && matrix[i + 1][j - 1] == 'M'
                        && matrix[i + 1][j + 1] == 'M',
                );
            }
        }
    }

    count
}

fn load_input() -> AnyResult<Vec<Vec<char>>> {
    let file = File::open("input/day04.txt").context("open input file")?;

    std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.context("read input line")?;
            Ok(line.chars().collect::<Vec<_>>())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let matrix = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let result = solve_part1(&matrix);

        assert_eq!(result, 18);
    }

    #[test]
    fn part2() {
        let matrix = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let result = solve_part2(&matrix);

        assert_eq!(result, 9);
    }
}
