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

/// Moves Santa according to the elf's directions, and keep track of the visited
/// houses.
fn follow_instructions(instructions: &str, houses: &mut HashSet<Point>) {
    let mut santa = Point { x: 0, y: 0 };

    houses.insert(santa);
    for dir in instructions.chars().map(|c| Direction::from(c)) {
        santa.mv(dir);
        houses.insert(santa);
    }
}


fn main() {
    let file = File::open("input.txt").expect("cannot open input.txt");
    let mut input  = String::new();
    let mut houses = HashSet::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());

    follow_instructions(&input, &mut houses);
    println!("{} houses received at least one present.", houses.len());
}

// {{{ Tests

#[test]
fn examples_part1() {
    let mut houses = HashSet::new();
    follow_instructions(">", &mut houses);
    assert_eq!(houses.len(), 2);

    houses.clear();
    follow_instructions("^>v<", &mut houses);
    assert_eq!(houses.len(), 4);

    houses.clear();
    follow_instructions("^v^v^v^v^v", &mut houses);
    assert_eq!(houses.len(), 2);
}

// }}}
