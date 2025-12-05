/// Solution for day 4, part 1.
#[must_use]
pub fn solve_part1(grid: Vec<Vec<bool>>) -> usize {
    Map::new(grid).pick_paper()
}

/// Solution for day 4, part 2.
#[must_use]
pub fn solve_part2(grid: Vec<Vec<bool>>) -> usize {
    let mut map = Map::new(grid);
    let mut count = 0;

    loop {
        let update = map.pick_paper();
        if update == 0 {
            break;
        }
        count += update;
    }

    count
}

struct Map {
    grid: Vec<Vec<bool>>,
    h: usize,
    w: usize,
}

impl Map {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let h = grid.len();
        let w = grid[0].len();
        Self { grid, h, w }
    }

    #[expect(
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        reason = "non-issue with our range"
    )]
    fn is_reachable(&self, y: usize, x: usize, dy: isize, dx: isize) -> bool {
        let ny = y as isize + dy;
        let nx = x as isize + dx;

        if ny < 0 || ny >= self.h as isize || nx < 0 || nx >= self.w as isize {
            return false;
        }

        self.grid[ny as usize][nx as usize]
    }

    #[expect(clippy::needless_range_loop, reason = "more readable that way")]
    fn pick_paper(&mut self) -> usize {
        let mut count = 0;
        let mut next = self.grid.clone();

        for y in 0..self.h {
            for x in 0..self.w {
                if !self.grid[y][x] {
                    continue;
                }

                let neighbors = usize::from(self.is_reachable(y, x, -1, -1))
                    + usize::from(self.is_reachable(y, x, -1, 0))
                    + usize::from(self.is_reachable(y, x, -1, 1))
                    + usize::from(self.is_reachable(y, x, 0, -1))
                    + usize::from(self.is_reachable(y, x, 0, 1))
                    + usize::from(self.is_reachable(y, x, 1, -1))
                    + usize::from(self.is_reachable(y, x, 1, 0))
                    + usize::from(self.is_reachable(y, x, 1, 1));

                if neighbors < 4 {
                    next[y][x] = false;
                    count += 1;
                }
            }
        }
        self.grid = next;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context as _, Result as AnyResult};
    use std::{fs::File, io::prelude::*};

    fn load_input() -> AnyResult<Vec<Vec<bool>>> {
        let file = File::open("input/day04.txt").context("open input file")?;

        std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                let line = line.context("read input line")?;
                Ok(line.chars().map(|ch| ch == '@').collect::<Vec<_>>())
            })
            .collect()
    }

    #[test]
    fn example_part1() {
        let map = [
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|row| row.chars().map(|ch| ch == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let result = solve_part1(map);

        assert_eq!(result, 13);
    }

    #[test]
    fn real_part1() -> AnyResult<()> {
        let map = load_input().context("load input")?;
        let result = solve_part1(map);

        assert_eq!(result, 1457);

        Ok(())
    }

    #[test]
    fn example_part2() {
        let map = [
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|row| row.chars().map(|ch| ch == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let result = solve_part2(map);

        assert_eq!(result, 43);
    }

    #[test]
    fn real_part2() -> AnyResult<()> {
        let map = load_input().context("load input")?;
        let result = solve_part2(map);

        assert_eq!(result, 8310);

        Ok(())
    }
}
