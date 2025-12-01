use anyhow::{Context as _, Result as AnyResult};
use std::{collections::HashSet, fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let map = load_input().context("load input")?;

    let result = solve_part1(&map);
    println!("Part 1: {result}");
    assert_eq!(result, 607);

    let result = solve_part2(&map);
    println!("Part 2: {result}");
    assert_eq!(result, 1384);

    Ok(())
}

fn solve_part1(map: &Map) -> usize {
    let mut scores = vec![vec![0; map.w as usize]; map.h as usize];
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    for i in 0..map.h as usize {
        for j in 0..map.w as usize {
            if map.grid[i][j] == 9 {
                stack.clear();
                visited.clear();
                stack.push(Position::new(j, i));
                while let Some(pos) = stack.pop() {
                    let current_value = map.get(pos).expect("in-bound");
                    if visited.insert(pos) {
                        scores[pos.y as usize][pos.x as usize] += 1;
                        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                            let mut new_pos = pos;
                            new_pos.x += dx;
                            new_pos.y += dy;
                            if let Some(next_value) = map.get(new_pos)
                                && next_value == current_value - 1 {
                                    stack.push(new_pos)
                                }
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    #[expect(clippy::needless_range_loop, reason = "false positive")]
    for i in 0..map.h as usize {
        for j in 0..map.w as usize {
            if map.grid[i][j] == 0 {
                result += scores[i][j];
            }
        }
    }

    result
}

fn solve_part2(map: &Map) -> usize {
    let mut scores = vec![vec![0; map.w as usize]; map.h as usize];
    let mut stack = Vec::new();
    for i in 0..map.h as usize {
        for j in 0..map.w as usize {
            if map.grid[i][j] == 9 {
                stack.clear();
                stack.push(Position::new(j, i));
                while let Some(pos) = stack.pop() {
                    let current_value = map.get(pos).expect("in-bound");
                    scores[pos.y as usize][pos.x as usize] += 1;
                    for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                        let mut new_pos = pos;
                        new_pos.x += dx;
                        new_pos.y += dy;
                        if let Some(next_value) = map.get(new_pos)
                            && next_value == current_value - 1 {
                                stack.push(new_pos)
                            }
                    }
                }
            }
        }
    }

    let mut result = 0;
    #[expect(clippy::needless_range_loop, reason = "false positive")]
    for i in 0..map.h as usize {
        for j in 0..map.w as usize {
            if map.grid[i][j] == 0 {
                result += scores[i][j];
            }
        }
    }

    result
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Debug)]
struct Map {
    w: i32,
    h: i32,
    grid: Vec<Vec<u8>>,
}

impl Map {
    fn get(&self, pos: Position) -> Option<i32> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .map(|value| *value as i32)
    }
}

fn load_input() -> AnyResult<Map> {
    let file = File::open("input/day10.txt").context("open input file")?;
    let grid = std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.context("read line")?;
            let mut row = line.into_bytes();
            for value in &mut row {
                *value -= b'0';
            }
            Ok(row)
        })
        .collect::<AnyResult<Vec<_>>>()?;

    Ok(Map {
        w: grid[0].len() as i32,
        h: grid.len() as i32,
        grid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let map = Map {
            w: 8,
            h: 8,
            grid: vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2],
            ],
        };
        let result = solve_part1(&map);

        assert_eq!(result, 36);
    }

    #[test]
    fn part2() {
        let map = Map {
            w: 8,
            h: 8,
            grid: vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2],
            ],
        };
        let result = solve_part2(&map);

        assert_eq!(result, 81);
    }
}
