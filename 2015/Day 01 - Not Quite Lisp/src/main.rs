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

fn compute_floor(directions : &str) -> i32 {
    directions.chars().fold(0, |floor, c| {
        match c {
            '(' => floor + 1,
            ')' => floor - 1,
             _  => panic!("unexpected char: {}", c),
        }
    })
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());

    println!("{}", compute_floor(&input.trim()));
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

// }}}
