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

use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

// }}}
// {{{ Move

/// Represents a turn direction.
#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

/// Represents a movement instruction.
///
/// i.e a turn direction and how far to walk).
#[derive(Debug)]
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

/// Represents an agent working for Santa.
#[derive(Debug)]
struct Agent {
    position:  Point,
    direction: Direction,
    has_read_all: bool,
}

impl Agent {
    /// Drops the agent on the landing site.
    fn new(has_read_all: bool) -> Agent {
        Agent {
            position: Point { x: 0, y: 0 },
            direction: Direction::North,
            has_read_all: has_read_all,
        }
    }

    /// Moves according to the instructions.
    fn follow_instructions(&mut self, instructions: &Vec<Move>) {
        let mut blocks = HashSet::new();

        for instruction in instructions {
            self.direction = turn(self.direction, instruction.turn);
            for _ in 0 .. instruction.steps {
                // If we have read **all** the instructions, we know we should
                // stop as soon as we walk on the same block twice.
                if self.has_read_all {
                    if !blocks.insert(self.position) {
                        break;
                    }
                }
                match self.direction {
                    Direction::North => self.position.y += 1,
                    Direction::East  => self.position.x += 1,
                    Direction::South => self.position.y -= 1,
                    Direction::West  => self.position.x -= 1,
                }

            }
        }
    }
}

/// Represents a point's orientation.
#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

/// Represents a location on the map.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x:   i32,
    y:   i32,
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
fn follow_instructions(instructions: &str, has_read_all: bool) -> i32{
    // Parse the document's instructions.
    let moves: Vec<Move> = instructions.split(',')
                                       .map(|s| s.trim().parse().unwrap())
                                       .collect();
    // Follow the instructions.
    let start = Point { x: 0, y: 0 };
    let mut me = Agent::new(has_read_all);
    me.follow_instructions(&moves);
    // Compute the distance.
    manhattan_dist(start, me.position)
}

// }}}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = String::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());

    println!("Part 1: {}", follow_instructions(&input, false));
    println!("Part 2: {}", follow_instructions(&input, true));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(follow_instructions("R2, L3",         false),  5);
    assert_eq!(follow_instructions("R2, R2, R2",     false),  2);
    assert_eq!(follow_instructions("R5, L5, R5, R3", false), 12);
}

#[test]
fn examples_part2() {
    assert_eq!(follow_instructions("R8, R4, R4, R8", true), 4);
}

// }}}
