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
use std::io::Read;
use std::str::FromStr;

// }}}
// {{{ Triangle

/// A triangle, represented by the lengths of its three sides.
#[derive(Debug)]
struct Triangle {
    sides: [i32; 3],
}

impl FromStr for Triangle {
    type Err = String;

    /// Builds a `Triangle` from a string.
    ///
    /// The input string should contains three integers separated by whitespaces.
    fn from_str(s: &str) -> Result<Triangle, String> {
        let sides = parse_line(s);
        Triangle::new(&[sides[0], sides[1], sides[2]])
                 .ok_or("cannot build triangle with these sides".to_string())
    }
}

impl Triangle {
    /// Builds a new triangle from the lengths of three sides.
    ///
    /// This function checks that the sum of any two sides is larger than the
    /// remaining side. If this condition is not met, the triangle cannot be
    /// built and None is returned.
    fn new(sides: &[i32; 3]) -> Option<Triangle> {
        if sides[0] + sides[1] > sides[2] &&
           sides[1] + sides[2] > sides[0] &&
           sides[0] + sides[2] > sides[1]
        {
            return Some(Triangle { sides: *sides });
        }
        None
    }
}

// }}}
// {{{ Parsing

/// Parses a single line of input and returns a vector of three integers.
///
/// The input line must contains **exactly** three integers, separated by
/// spaces.
fn parse_line(line : &str) -> Vec<i32> {
    let sides: Vec<i32> = line.split(' ')
                          .filter(|token| !token.is_empty())
                          .map(   |side|   side.parse().unwrap())
                          .collect();
    assert!(sides.len() == 3, "invalid number of columns");
    sides
}

/// Parses a list of triangles from a file organized by line.
///
///The input is expected to contains one triangle per line.
/// Example:
///
/// ```
/// triangle1a    triangle1b    triangle1c
/// triangle2a    triangle2b    triangle2c
/// triangle3a    triangle3b    triangle3c
/// ```
fn parse_by_line(input: &str) -> Vec<Triangle> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

/// Parses a list of triangles from a file organized by block.
///
/// The input is expected to be organized by block of three triangles.
/// Example:
///
/// ```
/// triangle1a    triangle3a    triangle3a
/// triangle1b    triangle3b    triangle3b
/// triangle1c    triangle3c    triangle3c
/// ```
fn parse_by_block(input: &str) -> Vec<Triangle> {
    let mut res   = Vec::new();
    let mut lines = 0;
    let mut rows  = input.lines().enumerate();
    // Staging buffer for triangles in construction.
    let mut sides = [[0, 0, 0]; 3];

    while let Some((side_idx, line)) = rows.next() {
        for (triangle_idx, val) in parse_line(&line).iter().enumerate() {
            sides[triangle_idx][side_idx % 3] = *val;
        }
        // If we got three sides, the triangles are complete.
        if side_idx % 3 == 2 {
            res.extend(&sides);
        }
        lines += 1;
    }
    assert!(lines % 3 == 0, "the last block is incomplete");
    res.iter().filter_map(|sides| Triangle::new(sides)).collect()
}

// }}}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    println!("Grouped by lines, there are {} valid triangles.",
             parse_by_line(&input).len());
    println!("Grouped by columns, there are {} valid triangles.",
             parse_by_block(&input).len());
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert!(Triangle::new(&[5, 10, 25]).is_none());
}

// }}}
