use anyhow::{Context as _, Result as AnyResult};
use std::{collections::HashSet, fs::File, io::prelude::*};

fn main() -> AnyResult<()> {
    let map = load_input().context("load input")?;

    let result = solve_part1(&map);
    println!("Part 1: {result}");
    assert_eq!(result, 1518548);

    Ok(())
}

fn solve_part1(map: &Map) -> usize {
    let plots = map.plots();

    plots
        .iter()
        .map(|plot| {
            let area = plot.len();
            let perimeter: usize = plot
                .iter()
                .map(|pos| {
                    let inner_sides = [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .iter()
                        .filter(|(dx, dy)| {
                            plot.contains(&Position {
                                x: pos.x + dx,
                                y: pos.y + dy,
                            })
                        })
                        .count();
                    4 - inner_sides
                })
                .sum();
            area * perimeter
        })
        .sum()
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
    fn get(&self, pos: Position) -> Option<u8> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .copied()
    }

    fn plots(&self) -> Vec<HashSet<Position>> {
        let mut plots = Vec::new();
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        for i in 0..self.h as usize {
            for j in 0..self.w as usize {
                let pos = Position::new(j, i);
                // Already visited (AKA part of an existing plot): skip.
                if visited.contains(&pos) {
                    continue;
                }
                // Start exploring a new plot, using DFS.
                let mut plot = HashSet::new();
                let plant = self.grid[i][j];
                stack.clear();
                stack.push(pos);
                while let Some(pos) = stack.pop() {
                    if visited.insert(pos) {
                        plot.insert(pos);
                        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                            let mut new_pos = pos;
                            new_pos.x += dx;
                            new_pos.y += dy;
                            if let Some(next_value) = self.get(new_pos)
                                && next_value == plant {
                                    stack.push(new_pos)
                                }
                        }
                    }
                }
                plots.push(plot);
            }
        }
        plots
    }
}

fn load_input() -> AnyResult<Map> {
    let file = File::open("input/day12.txt").context("open input file")?;
    let grid = std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.context("read line")?;
            Ok(line.into_bytes())
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
            w: 10,
            h: 10,
            grid: ["RRRRIICCFF",
                "RRRRIICCCF",
                "VVRRRCCFFF",
                "VVRCCCJFFF",
                "VVVVCJJCFE",
                "VVIVCCJJEE",
                "VVIIICJJEE",
                "MIIIIIJJEE",
                "MIIISIJEEE",
                "MMMISSJEEE"]
            .iter()
            .map(|&row| row.to_owned().into_bytes())
            .collect(),
        };
        let result = solve_part1(&map);

        assert_eq!(result, 1930);
    }
}
