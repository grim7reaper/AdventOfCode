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

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::Read;

// }}}
// {{{ Keypad

/// Represents a location on the keypad.
#[derive(Clone, Copy, Debug)]
struct Point {
    x:   i8,
    y:   i8,
}

/// A square keypad that controls the access to a room.
struct Keypad {
    keys: Vec<Option<char>>, // Keys layout.
    size: i8,                // Size of the keypad's side.
    key_5 : Point,           // Position of the key "5".
}

impl Keypad {
    /// Creates a keypad with the specified size and layout.
    fn new(keys: &[Option<char>], side: i8, starting_pos: Point) -> Keypad {
        assert_eq!((side*side) as usize, keys.len());

        let mut layout = Vec::with_capacity(keys.len());
        layout.extend(keys.iter().cloned());

        Keypad { keys: layout, size: side, key_5: starting_pos }
    }

    /// Return the starting position (i.e the position of the key "5").
    fn starting_pos(&self) -> Point {
        self.key_5
    }

    /// Checks if the position is valid.
    ///
    /// A position is valid if it is on the keypad, over a key.
    fn position_is_valid(&self, position: Point) -> bool {
        let idx = position.y * self.size + position.x;

        position.y >= 0 && position.y < self.size &&
        position.x >= 0 && position.x < self.size &&
        self.keys[idx as usize].is_some()
    }

    fn get_key_at(&self, position: Point) -> char {
        let idx = position.y * self.size + position.x;
        self.keys[idx as usize].unwrap()
    }
}

/// The keypad to unlock the bathroom.
lazy_static! {
    /// Represent the following keypad:
    /// ┌───────┐
    /// │1  2  3│
    /// │4  5  6│
    /// │7  8  9│
    /// └───────┘
    static ref SIMPLE_KEYPAD: Keypad = {
        Keypad::new(&[Some('1'), Some('2'), Some('3'),
                      Some('4'), Some('5'), Some('6'),
                      Some('7'), Some('8'), Some('9')],
                    3, Point { x: 1, y: 1 })
    };

    /// Represent the following keypad:
    /// ┌─────────────┐
    /// │      1      │
    /// │   2  3  4   │
    /// │5  6  7  8  9│
    /// │   A  B  C   │
    /// │      D      │
    /// └─────────────┘
    static ref WTF_KEYPAD: Keypad = {
        Keypad::new(&[  None,      None,    Some('1'),   None,      None,
                        None,    Some('2'), Some('3'), Some('4'),   None,
                      Some('5'), Some('6'), Some('7'), Some('8'), Some('9'),
                        None,    Some('A'), Some('B'), Some('C'),   None,
                        None,      None,    Some('D'),   None,      None],
                    5, Point { x: 0, y: 2 })
    };
}

// }}}
// {{{ Finger

/// Represents the finger's location on the keypad.
#[derive(Debug)]
struct Finger {
    position: Point,
}

impl Finger {
    /// Creates a finger.
    fn new(starting_pos: Point) -> Finger {
        Finger { position: starting_pos }
    }

    /// Move the finger over the keypad.
    fn mv(&mut self, movement: Move, keypad: &Keypad) {
        let mut target = self.position;

        match movement {
            Move::Up    => target.y -= 1,
            Move::Down  => target.y += 1,
            Move::Left  => target.x -= 1,
            Move::Right => target.x += 1,
        }
        if keypad.position_is_valid(target) {
            self.position = target;
        }
    }

    /// Returns the value of the key under the finger.
    fn press_key(&self, keypad: &Keypad) -> char {
        keypad.get_key_at(self.position)
    }
}

// }}}
// {{{ Move

/// Represents a movement of the finger.
///
/// The movement is relative to the current finger's position.
#[derive(Debug)]
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

fn follow_instructions(instructions: &str, keypad: &Keypad) -> String {
    let mut code  = String::new();
    let mut pinky = Finger::new(keypad.starting_pos());

    for line in instructions.lines() {
        let moves: Vec<Move> = line.chars().map(|c| Move::from(c)).collect();
        for movement in moves {
            pinky.mv(movement, keypad);
        }
        code.push(pinky.press_key(keypad));
    }
    code
}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    println!("The bathroom code should be {}.",
             follow_instructions(&input, &SIMPLE_KEYPAD));
    println!("Ho wait! On this crazy keypad, the code is {}.",
             follow_instructions(&input, &WTF_KEYPAD));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(follow_instructions("ULL\nRRDDD\nLURDL\nUUUUD", &SIMPLE_KEYPAD),
               "1985");
}

#[test]
fn examples_part2() {
    assert_eq!(follow_instructions("ULL\nRRDDD\nLURDL\nUUUUD", &WTF_KEYPAD),
               "5DB3");
}

// }}}
