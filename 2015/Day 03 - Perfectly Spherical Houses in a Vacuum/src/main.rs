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

// }}}
// {{{ Direction

/// Represents a direction.
#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Direction {
    /// Builds a `Direction` from a char.
    ///
    /// Valid inputs are ^, >, v and <.
    fn from(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
             _  => panic!("unexpected direction: {}", c),
        }
    }
}

// }}}
// {{{ Point

/// Represents the location of Santa.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Moves the point in the specified direction.
    fn mv(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East  => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West  => self.x -= 1,
        }
    }
}

// }}}

/// Moves Santa's crew according to the elf's directions, and keep track of the
/// visited houses.
fn follow_instructions(instructions: &str,
                       santa_crew:   &mut[Point],
                       houses:       &mut HashSet<Point>) {
    // Everyone starts at the same position, we only need to insert the first.
    houses.insert(santa_crew[0]);

    let instructions = instructions.chars().map(|c| Direction::from(c));
    for (crew, direction) in instructions.enumerate() {
        let worker_idx = crew % santa_crew.len();
        let worker     = &mut santa_crew[worker_idx];
        worker.mv(direction);
        houses.insert(*worker);
    }
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input.txt");
    let mut input  = String::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());

    let mut houses = HashSet::new();
    let mut santa  = [Point { x: 0, y: 0 }; 1];
    follow_instructions(&input, &mut santa, &mut houses);
    println!("{} houses received at least one present from Santa.",
             houses.len());

    houses.clear();
    let mut santa_and_bot = [Point { x: 0, y: 0 }; 2];
    follow_instructions(&input, &mut santa_and_bot, &mut houses);
    println!("{} houses received at least one present from Santa's crew.",
             houses.len());
}

// {{{ Tests

#[test]
fn examples_part1() {
    let mut houses = HashSet::new();
    let mut santa  = [Point { x: 0, y: 0 }; 1];
    follow_instructions(">", &mut santa, &mut houses);
    assert_eq!(houses.len(), 2);

    houses.clear();
    let mut santa  = [Point { x: 0, y: 0 }; 1];
    follow_instructions("^>v<", &mut santa, &mut houses);
    assert_eq!(houses.len(), 4);

    houses.clear();
    let mut santa  = [Point { x: 0, y: 0 }; 1];
    follow_instructions("^v^v^v^v^v", &mut santa, &mut houses);
    assert_eq!(houses.len(), 2);
}

#[test]
fn examples_part2() {
    let mut houses     = HashSet::new();
    let mut santa_crew = [Point { x: 0, y: 0 }; 2];
    follow_instructions("^>", &mut santa_crew, &mut houses);
    assert_eq!(houses.len(), 3);

    houses.clear();
    let mut santa_crew = [Point { x: 0, y: 0 }; 2];
    follow_instructions("^>v<", &mut santa_crew, &mut houses);
    assert_eq!(houses.len(), 3);

    houses.clear();
    let mut santa_crew = [Point { x: 0, y: 0 }; 2];
    follow_instructions("^v^v^v^v^v", &mut santa_crew, &mut houses);
    assert_eq!(houses.len(), 11);
}

// }}}
