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

// }}}

/// Moves the next floor, according to the instruction.
///
/// The returned value is the next floor.
fn move_to_next_floor(current_floor: i32, instruction: char) -> i32 {
    match instruction {
        '(' => current_floor + 1,
        ')' => current_floor - 1,
        _  => panic!("unexpected instruction: `{}`", instruction),
    }
}

fn move_to_next_floor_mut(current_floor: &mut i32, instruction: char)
    -> Option<i32>
{
    match instruction {
        '(' => *current_floor += 1,
        ')' => *current_floor -= 1,
        _  => panic!("unexpected instruction: `{}`", instruction),
    };
    Some(*current_floor)
}

/// Computes the destination floor.
fn compute_floor(directions : &str) -> i32 {
    directions.chars().fold(0, move_to_next_floor)
}

/// Finds the first instruction that causes us to enter the basement (floor -1).
fn find_instruction_to_basement(directions : &str) -> usize {
    directions.chars().scan(0, move_to_next_floor_mut)
                      .take_while(|floor | *floor != -1)
                      .count() + 1
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());

    println!("Santa should go to the floor {}.", compute_floor(&input.trim()));
    println!("The first time Santa goes into the basement it at instruction {}.",
             find_instruction_to_basement(&input.trim()));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(compute_floor("(())"),     0);
    assert_eq!(compute_floor("()()"),     0);
    assert_eq!(compute_floor("((("),      3);
    assert_eq!(compute_floor("(()(()("),  3);
    assert_eq!(compute_floor("))((((("),  3);
    assert_eq!(compute_floor("())"),     -1);
    assert_eq!(compute_floor("))("),     -1);
    assert_eq!(compute_floor(")))"),     -3);
    assert_eq!(compute_floor(")())())"), -3);
}

#[test]
fn examples_part2() {
    assert_eq!(find_instruction_to_basement(")"),     1);
    assert_eq!(find_instruction_to_basement("()())"), 5);
}

// }}}
