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

#[macro_use]
extern crate nom;

// }}}
// {{{ Basic types

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x : u16,
    y : u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    tl : Point, // Coordinate of the top-left corner.
    br : Point, // Coordinate of the bottom-right corner.
}

// }}}
// {{{ Grid

mod grid {
    #[derive(Debug)]
    pub struct Grid {
        width  : usize,
        height : usize,
        lights : Vec<bool>,
    }

    impl Grid {
        /// Creates a new grid with all the lights turned off.
        pub fn new(width: usize, height: usize) -> Self {
            Grid {
                width:  width,
                height: height,
                lights: vec![false; width*height],
            }
        }

        /// Returns the number of lights lit on the grid.
        pub fn lights_lit(&self) -> i32 {
            self.lights.iter().fold(0, |sum, &p| if p { sum + 1 } else { sum })
        }

        /// Turns on the lights in the specified rectangle.
        pub fn turn_on(&mut self, rect: &super::Rect) {
            self.apply_op_on_rect(|_| true, rect);
        }

        /// Turns off the lights in the specified rectangle.
        pub fn turn_off(&mut self, rect: &super::Rect) {
            self.apply_op_on_rect(|_| false, rect);
        }

        /// Toggle the lights in the specified rectangle.
        pub fn toggle(&mut self, rect: &super::Rect) {
            self.apply_op_on_rect(|b| !b, rect);
        }

        fn apply_op_on_rect<F>(&mut self, op: F, area: &super::Rect)
            where F: Fn(bool) -> bool {
            let i_0 = area.tl.x as usize;
            let i_n = area.br.x as usize;
            let j_0 = area.tl.y as usize;
            let j_n = area.br.y as usize;

            for i in i_0 .. i_n + 1 {
                for j in j_0 .. j_n + 1 {
                    let idx = indices_to_index(i, j, self.width);
                    self.lights[idx] = op(self.lights[idx]);
                }
            }
        }
    }
    // {{{ Helpers

    /// Converts 2D indices into a 1D index.
    #[inline(always)]
    fn indices_to_index(line: usize, column: usize, width: usize) -> usize {
        line*width + column
    }

    // }}}
}

// }}}
// {{{ Instruction

mod instruction {
    use std::str;
    use nom;

    #[derive(Clone, Copy, Debug)]
    pub enum Instruction {
        TurnOn(super::Rect),
        TurnOff(super::Rect),
        Toggle(super::Rect)
    }

    impl str::FromStr for Instruction {
        type Err = nom::ErrorKind;

        /// Builds an `Instruction` from a string.
        fn from_str(s: &str) -> Result<Instruction, nom::ErrorKind> {
            parse_instruction(s.as_bytes()).to_result()
        }
    }

    // {{{ Parser

    /// Parse a point.
    named!(point<&[u8], super::Point>,
        do_parse!(
           x: map_res!(nom::digit, str::from_utf8)  >>
           tag!(",")                                >>
           y: map_res!(nom::digit, str::from_utf8)  >>
           (super::Point {
               x: x.parse::<u16>().unwrap(),
               y: y.parse::<u16>().unwrap()
           })
       )
    );

    /// Parse a rectangle.
    named!(rect<&[u8], super::Rect>,
        do_parse!(
            tl : point        >>
            tag!(" through ") >>
            br : point        >>
            (super::Rect { tl: tl, br: br })
        )
    );

    /// Parse the instruction to turn on a rectangle of lights.
    named!(turn_on<&[u8], super::Rect>,
        do_parse!(
           tag!("turn on ") >>
           r: rect          >>
           (r)
       )
    );

    /// Parse the instruction to turn off a rectangle of lights.
    named!(turn_off<&[u8], super::Rect>,
        do_parse!(
           tag!("turn off ") >>
           r: rect          >>
           (r)
       )
    );

    /// Parse the instruction to turn off a rectangle of lights.
    named!(toggle<&[u8], super::Rect>,
        do_parse!(
           tag!("toggle ") >>
           r: rect          >>
           (r)
       )
    );

    /// Parse an instruction.
    named!(parse_instruction<&[u8], Instruction>,
        alt!(
            turn_on  => {|r| Instruction::TurnOn(r) }
          | turn_off => {|r| Instruction::TurnOff(r)}
          | toggle   => {|r| Instruction::Toggle(r) }
        )
    );

    // }}}
}

// }}}

use grid::Grid;
use instruction::Instruction;

pub fn execute(grid: &mut Grid, instructions: &[Instruction]) {
    for instruction in instructions.iter() {
        match instruction {
            &Instruction::TurnOn(area)  => grid.turn_on(&area),
            &Instruction::TurnOff(area) => grid.turn_off(&area),
            &Instruction::Toggle(area)  => grid.toggle(&area),
        }
    }
}


fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap())
                                    .collect::<Vec<_>>();
    let mut grid = Grid::new(1000, 1000);
    execute(&mut grid, &instructions);
    println!("After following the instructions, there are {} lights lit.",
             grid.lights_lit());
}

// {{{ Tests

#[test]
fn examples_part1() {
    let mut grid = Grid::new(1000, 1000);

    grid.turn_on(&Rect { tl: Point { x: 0, y: 0}, br: Point { x: 999, y: 999}});
    assert_eq!(grid.lights_lit(), 1_000_000);

    grid.toggle(&Rect { tl: Point { x: 0, y: 0}, br: Point { x: 999, y: 0}});
    assert_eq!(grid.lights_lit(), 999_000);

    grid.turn_off(&Rect {
        tl: Point { x: 499, y: 499},
        br: Point { x: 500, y: 500}
    });
    assert_eq!(grid.lights_lit(), 998_996);
}

// }}}
