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
#[derive(Clone, Copy, Debug)]
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
        Ok(Triangle { sides: [sides[0], sides[1], sides[2]] })
    }
}

impl Triangle {
    /// Tests if a triangle is possible.
    ///
    /// A triangle is possible if the sum of any two sides is larger than the
    /// remaining side.
    fn is_possible(&self) -> bool {
        self.sides[0] + self.sides[1] > self.sides[2] &&
        self.sides[1] + self.sides[2] > self.sides[0] &&
        self.sides[0] + self.sides[2] > self.sides[1]
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
    input.lines().map(|s| s.parse().unwrap()).collect()
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
    let mut triangles = [Triangle { sides: [0, 0, 0] }; 3];

    while let Some((side_idx, line)) = rows.next() {
        for (triangle_idx, val) in parse_line(&line).iter().enumerate() {
            triangles[triangle_idx].sides[side_idx % 3] = *val;
        }
        // If we got three sides, the triangles are complete.
        if side_idx % 3 == 2 {
            res.extend(&triangles);
        }
        lines += 1;
    }
    assert!(lines % 3 == 0, "the last block is incomplete");
    res
}


// }}}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let count = parse_by_line(&input).iter().filter(|t| t.is_possible()).count();
    println!("{}", count);
    let count = parse_by_block(&input).iter().filter(|t| t.is_possible()).count();
    println!("{}", count);
}

// {{{ Tests

#[test]
fn examples_part1() {
    let t = Triangle { a: 5, b: 10, c: 25 };
    assert_eq!(t.is_possible(), false);
}

// }}}
