// {{{ Lints

#![deny(missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unstable_features,
        unused_import_braces,
        unused_qualifications
)]

// }}}
// {{{ Crates

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

// }}}
// {{{ Move

/// Represents a turn direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

/// Represents a movement instruction.
///
/// i.e a turn direction and how far to walk).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Move {
    turn:  Turn,
    steps: i32,
}

impl FromStr for Move {
    type Err = ();

    /// Builds a `Move` from a string.
    ///
    /// The input string should match the following regexp: "^(L|R)\d+$".
    fn from_str(s: &str) -> Result<Move, ()> {
        let mut chars = s.chars();

        let turn = match chars.nth(0).unwrap() {
            'L' => Turn::Left,
            'R' => Turn::Right,
            c   => panic!("unexpected turn direction: {}", c),
        };
        let steps: i32 = chars.as_str().parse().unwrap();

        Ok(Move { turn: turn, steps: steps })
    }
}

// }}}
// {{{ Point

/// Represents a point's orientation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

/// Represents a location on the map.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    dir: Direction,
    x:   i32,
    y:   i32,
}

impl Point {
    /// Moves the point according to the instruction.
    fn walk(&mut self, instruction: Move) {
        self.dir = turn(self.dir, instruction.turn);
        match self.dir {
            Direction::North => self.y += instruction.steps,
            Direction::East  => self.x += instruction.steps,
            Direction::South => self.y -= instruction.steps,
            Direction::West  => self.x -= instruction.steps,
        }
    }
}

// }}}
// {{{ Helpers

/// Returns the new direction after turning.
fn turn(dir: Direction, turn: Turn) -> Direction {
    match turn {
        Turn::Left =>
            match dir {
                Direction::North => Direction::West,
                Direction::East  => Direction::North,
                Direction::South => Direction::East,
                Direction::West  => Direction::South,
            },

        Turn::Right =>
            match dir {
                Direction::North => Direction::East,
                Direction::East  => Direction::South,
                Direction::South => Direction::West,
                Direction::West  => Direction::North,
            },
    }
}

/// Computes the Manhattan distance between two points.
fn manhattan_dist(p: Point, q: Point) -> i32 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

/// Computes the distance between our landing site and the Easter Bunny HQ.
fn compute_shortest_path_to_hq(instructions: &str) -> i32{
    // Parse the document's instructions.
    let moves: Vec<Move> = instructions.split(',')
                                       .map(|s| s.trim().parse().unwrap())
                                       .collect();
    // Follow the instructions.
    let start = Point { dir: Direction::North, x: 0, y: 0 };
    let mut santa = start;
    for instruction in moves {
        santa.walk(instruction);
    }
    // Compute the distance.
    manhattan_dist(start, santa)
}

// }}}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = String::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());

    println!("{}", compute_shortest_path_to_hq(&input));
}

// {{{ Tests

#[test]
fn examples() {
    assert_eq!(compute_shortest_path_to_hq("R2, L3"),          5);
    assert_eq!(compute_shortest_path_to_hq("R2, R2, R2"),      2);
    assert_eq!(compute_shortest_path_to_hq("R5, L5, R5, R3"), 12);
}

// }}}
