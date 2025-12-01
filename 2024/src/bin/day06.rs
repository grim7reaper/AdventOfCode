use anyhow::{Context as _, Result as AnyResult};
use rayon::prelude::*;
use std::{collections::HashSet, fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let input = load_input().context("load input")?;

    let result = solve_part1(&input);
    println!("Part 1: {result}");
    assert_eq!(result, 4758);

    let result = solve_part2(&input);
    println!("Part 2: {result}");
    assert_eq!(result, 1670);

    Ok(())
}

fn solve_part1(input: &Input) -> usize {
    walk(&input.map, input.start).len()
}

fn walk(map: &[Vec<bool>], start: (i32, i32)) -> HashSet<(i32, i32)> {
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let mut y = start.0;
    let mut x = start.1;
    let mut dy = -1;
    let mut dx = 0;
    let mut visited = HashSet::new();
    visited.insert(start);

    while (0..h).contains(&y) && (0..w).contains(&x) {
        if map[y as usize][x as usize] {
            // Hit an obstacle, bounce back.
            x -= dx;
            y -= dy;
            // Rotate.
            match (dy, dx) {
                (-1, 0) => {
                    dy = 0;
                    dx = 1;
                }
                (0, 1) => {
                    dy = 1;
                    dx = 0;
                }
                (1, 0) => {
                    dy = 0;
                    dx = -1;
                }
                (0, -1) => {
                    dy = -1;
                    dx = 0;
                }
                _ => unreachable!(),
            }
        } else {
            visited.insert((y, x));
        }
        y += dy;
        x += dx;
    }

    visited
}

fn solve_part2(input: &Input) -> usize {
    walk(&input.map, input.start)
        .into_par_iter()
        .filter(|(y, x)| {
            if (*y, *x) == input.start {
                return false;
            }
            let x = *x as usize;
            let y = *y as usize;

            let mut map = input.map.clone();
            map[y][x] = true;
            detect_loop(&map, input.start)
        })
        .count()
}

fn detect_loop(map: &[Vec<bool>], start: (i32, i32)) -> bool {
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let mut y = start.0;
    let mut x = start.1;
    let mut dy = -1;
    let mut dx = 0;
    let mut visited = HashSet::new();

    while (0..h).contains(&y) && (0..w).contains(&x) {
        if map[y as usize][x as usize] {
            // Hit an obstacle, bounce back.
            x -= dx;
            y -= dy;
            // Rotate.
            match (dy, dx) {
                (-1, 0) => {
                    dy = 0;
                    dx = 1;
                }
                (0, 1) => {
                    dy = 1;
                    dx = 0;
                }
                (1, 0) => {
                    dy = 0;
                    dx = -1;
                }
                (0, -1) => {
                    dy = -1;
                    dx = 0;
                }
                _ => unreachable!(),
            }
        } else if !visited.insert(((y, x), (dy, dx))) {
            return true;
        }
        y += dy;
        x += dx;
    }

    false
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<bool>>,
    start: (i32, i32),
}

fn load_input() -> AnyResult<Input> {
    let file = File::open("input/day06.txt").context("open input file")?;
    std::io::BufReader::new(file).lines().enumerate().try_fold(
        Input {
            map: Vec::new(),
            start: (0, 0),
        },
        |mut state, (i, line)| {
            let line = line.context("read line")?;
            state.map.push(line.chars().map(|ch| ch == '#').collect());
            if let Some(j) = line.find('^') {
                state.start = (i as i32, j as i32);
            }
            Ok(state)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        #[rustfmt::skip]
        let map = vec![
            vec![false, false, false, false, true,  false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, true,  false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, true,  false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, true,  false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, true,  false],
            vec![true,  false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, true,  false, false, false],
        ];
        let start = (6, 4);
        let input = Input { map, start };
        let result = solve_part1(&input);

        assert_eq!(result, 41);
    }

    #[test]
    fn part2() {
        #[rustfmt::skip]
        let map = vec![
            vec![false, false, false, false, true,  false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, true,  false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, true,  false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, true,  false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, true,  false],
            vec![true,  false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, true,  false, false, false],
        ];
        let start = (6, 4);
        let input = Input { map, start };
        let result = solve_part2(&input);

        assert_eq!(result, 6);
    }
}
