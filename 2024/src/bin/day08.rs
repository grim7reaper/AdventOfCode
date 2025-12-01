use anyhow::{Context as _, Result as AnyResult};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::prelude::*,
};

fn main() -> AnyResult<()> {
    let map = load_input().context("load input")?;

    let result = solve_part1(&map);
    println!("Part 1: {result}");
    assert_eq!(result, 329);

    let result = solve_part2(&map);
    println!("Part 2: {result}");
    assert_eq!(result, 1190);

    Ok(())
}

fn solve_part1(map: &Map) -> usize {
    map.antennas
        .values()
        .fold(HashSet::new(), |mut acc, positions| {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let src = positions[i];
                    let dst = positions[j];
                    let dx = dst.x - src.x;
                    let dy = dst.y - src.y;

                    let anti_src = Position {
                        x: src.x - dx,
                        y: src.y - dy,
                    };
                    let anti_dst = Position {
                        x: dst.x + dx,
                        y: dst.y + dy,
                    };
                    if map.position_is_valid(anti_src) {
                        acc.insert(anti_src);
                    }
                    if map.position_is_valid(anti_dst) {
                        acc.insert(anti_dst);
                    }
                }
            }
            acc
        })
        .len()
}

fn solve_part2(map: &Map) -> usize {
    map.antennas
        .values()
        .fold(HashSet::new(), |mut acc, positions| {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let src = positions[i];
                    let dst = positions[j];
                    let dx = dst.x - src.x;
                    let dy = dst.y - src.y;

                    let mut anti_src = src;
                    while map.position_is_valid(anti_src) {
                        acc.insert(anti_src);
                        anti_src.x -= dx;
                        anti_src.y -= dy;
                    }
                    let mut anti_dst = dst;
                    while map.position_is_valid(anti_dst) {
                        acc.insert(anti_dst);
                        anti_dst.x += dx;
                        anti_dst.y += dy;
                    }
                }
            }
            acc
        })
        .len()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Map {
    w: i32,
    h: i32,
    antennas: HashMap<u8, Vec<Position>>,
}

impl Map {
    fn position_is_valid(&self, pos: Position) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }

    // fn dump(&self, antinodes: &HashSet<Position>) {
    //     let mut map = vec![vec![b'.'; self.w as usize]; self.h as usize];
    //     for (symbol, positions) in &self.antennas {
    //         for pos in positions {
    //             map[pos.y as usize][pos.x as usize] = *symbol;
    //         }
    //     }
    //     for pos in antinodes {
    //         map[pos.y as usize][pos.x as usize] = b'#';
    //     }
    //     println!(
    //         "{}",
    //         map.into_iter()
    //             .map(|row| String::from_utf8_lossy(&row).into_owned())
    //             .collect::<Vec<_>>()
    //             .join("\n")
    //     );
    // }
}

fn load_input() -> AnyResult<Map> {
    let file = File::open("input/day08.txt").context("open input file")?;
    std::io::BufReader::new(file)
        .lines()
        .try_fold(
            (0, 0, HashMap::<u8, Vec<_>>::new()),
            |(_, h, mut antennas), line| {
                let line = line.context("read line")?;
                let row = line.as_bytes();
                for (idx, ch) in row.iter().enumerate() {
                    if ch.is_ascii_alphanumeric() {
                        antennas.entry(*ch).or_default().push(Position {
                            y: h,
                            x: idx as i32,
                        });
                    }
                }
                Ok((row.len() as i32, h + 1, antennas))
            },
        )
        .map(|(w, h, antennas)| Map { w, h, antennas })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let map = [
            (48, vec![(8, 1), (5, 2), (7, 3), (4, 4)]),
            (65, vec![(6, 5), (8, 8), (9, 9)]),
        ]
        .iter()
        .fold(
            Map {
                w: 12,
                h: 12,
                antennas: HashMap::new(),
            },
            |mut map, (ch, pos)| {
                let pos = pos
                    .iter()
                    .copied()
                    .map(|(x, y)| Position { x, y })
                    .collect::<Vec<_>>();
                map.antennas.insert(*ch, pos);
                map
            },
        );
        let result = solve_part1(&map);

        assert_eq!(result, 14);
    }

    #[test]
    fn part2() {
        let map = [
            (48, vec![(8, 1), (5, 2), (7, 3), (4, 4)]),
            (65, vec![(6, 5), (8, 8), (9, 9)]),
        ]
        .iter()
        .fold(
            Map {
                w: 12,
                h: 12,
                antennas: HashMap::new(),
            },
            |mut map, (ch, pos)| {
                let pos = pos
                    .iter()
                    .copied()
                    .map(|(x, y)| Position { x, y })
                    .collect::<Vec<_>>();
                map.antennas.insert(*ch, pos);
                map
            },
        );
        let result = solve_part2(&map);

        assert_eq!(result, 34);
    }
}
