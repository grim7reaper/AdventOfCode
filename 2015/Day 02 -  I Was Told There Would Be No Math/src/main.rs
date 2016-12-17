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

    /// Computes the ribbon length required to wrap a present (in feet).
    ///
    /// The ribbon length required for a present is the shortest distance around
    /// its sides, or the smallest perimeter of any one face.
    /// Each present also requires a bow made out of ribbon as well; the feet of
    /// ribbon required for the perfect bow is equal to the cubic feet of volume
    /// of the present.
    fn ribbon_length(&self) -> i32 {
        let perimeters = [
            2*self.sides[0] + 2*self.sides[1],
            2*self.sides[1] + 2*self.sides[2],
            2*self.sides[0] + 2*self.sides[2]
        ];
        let smallest = *perimeters.iter().min().unwrap();
        self.sides.iter().product::<i32>() + smallest
    }
}

// }}}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let (paper, ribbon) = input.lines()
                               .map(|l| l.parse::<Present>().unwrap())
                               .map(|p| (p.package_area(), p.ribbon_length()))
                               .fold((0, 0), |sum, v| (sum.0+v.0, sum.1+v.1));
    println!("The elves should order {} square feet of wrapping paper.", paper);
    println!("The elves should order {} feet of ribbon.", ribbon);
}

// {{{ Tests

#[test]
fn examples() {
    let present = "2x3x4".parse::<Present>();
    assert!(present.is_ok());
    assert_eq!(present.as_ref().unwrap().package_area(),  58);
    assert_eq!(present.as_ref().unwrap().ribbon_length(), 34);

    let present = "1x1x10".parse::<Present>();
    assert!(present.is_ok());
    assert_eq!(present.as_ref().unwrap().package_area(),  43);
    assert_eq!(present.as_ref().unwrap().ribbon_length(), 14);
}

// }}}
