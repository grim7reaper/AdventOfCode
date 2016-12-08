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

use std::cmp;
use std::fs::File;
use std::io::Read;

// }}}
// {{{ Finger

/// The keypad to unlock the bathroom.
const KEYPAD: [[i32; 3]; 3] = [[1, 2, 3],
                               [4, 5, 6],
                               [7, 8, 9]];

/// Represents the finger's location on the keypad.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Finger {
    x: i8,
    y: i8,
}

impl Finger {
    /// Creates a finger at the initial position (over the 5).
    fn new() -> Finger {
        Finger { x: 0, y: 0 }
    }

    /// Move the finger over the keypad.
    ///
    /// The keypad is represented as follow:
    ///
    ///   -1  0 +1
    ///   ┌───────┐
    /// -1│1  2  3│
    ///  0│4  5  6│
    /// +1│7  8  9│
    ///   └───────┘
    fn mv(&mut self, movement: Move) {
        match movement {
            Move::Up    => self.y = cmp::max(self.y - 1, -1),
            Move::Down  => self.y = cmp::min(self.y + 1,  1),
            Move::Left  => self.x = cmp::max(self.x - 1, -1),
            Move::Right => self.x = cmp::min(self.x + 1,  1),
        }
    }

    /// Returns the value of the key under the finger.
    fn press_key(&self) -> i32 {
        // +1 because indexing starts at 0, not -1.
        let i = self.y + 1;
        let j = self.x + 1;

        KEYPAD[i as usize][j as usize]
    }
}

// }}}
// {{{ Move

/// Represents a movement of the finger.
///
/// The movement is relative to the current finger's position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    /// Builds a Move from a single char.
    ///
    /// Valid inputs are U, D, L and R.
    fn from(c: char) -> Move {
        match c {
            'U' => Move::Up,
            'D' => Move::Down,
            'L' => Move::Left,
            'R' => Move::Right,
            _   => panic!("invalid move: {}", c),
        }
    }
}

// }}}

fn follow_instructions(instructions: &str) -> i32 {
    let mut code = 0;
    let mut pinky = Finger::new();

    for line in instructions.lines() {
        let moves: Vec<Move> = line.chars().map(|c| Move::from(c)).collect();
        for movement in moves {
            pinky.mv(movement);
        }
        code = code*10 + pinky.press_key();
    }
    code
}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    println!("{}", follow_instructions(&input));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(follow_instructions("ULL\nRRDDD\nLURDL\nUUUUD"), 1985);
}

// }}}
