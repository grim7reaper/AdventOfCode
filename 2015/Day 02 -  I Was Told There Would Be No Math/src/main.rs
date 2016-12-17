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
// {{{ Presents

/// A present, represented by the surfaces of its three sides (in square feet).
#[derive(Debug)]
struct Present {
    sides: [i32; 3],
}

impl FromStr for Present {
    type Err = String;

    /// Builds a `Present` from a string.
    ///
    /// The input string should contains three integers separated by x.
    /// Example: 2x3x4
    fn from_str(s: &str) -> Result<Present, String> {
        let sides: Vec<i32> = s.split('x')
                              .map(|side| side.parse().unwrap())
                              .collect();
        assert!(sides.len() == 3, "invalid number of dimensions");
        Ok(Present { sides: [sides[0], sides[1], sides[2]] })
    }
}

impl Present {
    /// Computes the paper area required to wrap the present (in square feet).
    ///
    /// The wrapping paper area required for a present is 2*l*w + 2*w*h + 2*h*l.
    /// A little extra paper (the area of the smallest side) for each present is
    /// also required.
    fn package_area(&self) -> i32 {
        let areas = [
            self.sides[0]*self.sides[1],
            self.sides[1]*self.sides[2],
            self.sides[0]*self.sides[2]
        ];
        let smallest = *areas.iter().min().unwrap();
        areas.iter().map(|x| 2 * x).sum::<i32>() + smallest
    }
}

// }}}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let total = input.lines().map(|l| l.parse::<Present>().unwrap())
                             .map(|p| p.package_area())
                             .sum::<i32>();
    println!("{}", total);
}

// {{{ Tests

#[test]
fn examples_part1() {
    let present = "2x3x4".parse::<Present>();
    assert!(present.is_ok());
    assert_eq!(present.unwrap().package_area(), 58);

    let present = "1x1x10".parse::<Present>();
    assert!(present.is_ok());
    assert_eq!(present.unwrap().package_area(), 43);
}

// }}}
