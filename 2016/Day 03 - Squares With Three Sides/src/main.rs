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

#[derive(Clone, Copy, Debug)]
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl FromStr for Triangle {
    type Err = String;

    /// Builds a `Triangle` from a string.
    ///
    /// The input string should contains three integers separated by whitespaces.
    fn from_str(s: &str) -> Result<Triangle, String> {
        let sides: Vec<i32> = s.split(' ')
                              .filter(|token| !token.is_empty())
                              .map(   |side|   side.parse().unwrap())
                              .collect();

        if sides.len() != 3 {
            return Err(format!("expected 3 sides, got {}", sides.len()));
        }
        Ok(Triangle { a: sides[0], b: sides[1], c: sides[2] })
    }
}

impl Triangle {
    /// Tests if a triangle is possible.
    ///
    /// A triangle is possible if the sum of any two sides is larger than the
    /// remaining side.
    fn is_possible(&self) -> bool {
        self.a + self.b > self.c &&
        self.b + self.c > self.a &&
        self.a + self.c > self.b
    }
}

// }}}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let count = input.lines()
                     .map(   |s| s.parse::<Triangle>().unwrap())
                     .filter(|t| t.is_possible()).count();
    println!("{}", count);
}

// {{{ Tests

#[test]
fn examples_part1() {
    let t = Triangle { a: 5, b: 10, c: 25 };
    assert_eq!(t.is_possible(), false);
}

// }}}
