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
use nom::digit;

// }}}
// {{{ Screen

mod screen {
    use std::fmt;

    #[derive(Debug)]
    pub struct Screen {
        width  : u16,
        height : u16,
        pixels : Vec<bool>,
    }

    impl Screen {
        /// Creates a new screen with all the pixels turned off.
        pub fn new(width: u16, height: u16) -> Self {
            Screen {
                width:  width,
                height: height,
                pixels: vec![false; (width*height) as usize],
            }
        }

        /// Returns the number of pixels lit on the screen.
        pub fn pixels_lit(&self) -> i32 {
            self.pixels.iter().fold(0, |sum, &p| if p { sum + 1 } else { sum })
        }

        /// Turns on the pixels in a rectangle at the top-left of the screen.
        pub fn turn_on(&mut self, width: u16, height: u16) {
            for i in 0 .. height {
                for j in 0 .. width {
                    let idx = indices_to_index(i, j, self.width);
                    self.pixels[idx] = true;
                }
            }
        }

        /// Rotates the row `row` by `shift` pixels.
        ///
        /// This function shifts all of the pixels in row `row` (0 is the top
        /// row) right by `shift` pixels.
        /// Pixels that would fall off the right end appear at the left end of
        /// the row.
        pub fn rotate_row(&mut self, row: usize, shift: usize) {
            let start = row * (self.width as usize);
            let len   = self.width as usize;
            self.rotate(start, len, shift, 1);
        }

        /// Rotates the column `col` by `shift` pixels.
        ///
        /// This function shifts all of the pixels in column `col` (0 is the
        /// left column) down by `shift` pixels.
        /// Pixels that would fall off the bottom appear at the top of the
        /// column.
        pub fn rotate_col(&mut self, col: usize, shift: usize) {
            let start  = col;
            let len    = self.height as usize;
            let stride = self.width  as usize;
            self.rotate(start, len, shift, stride);
        }

        /// Rotates an array or the part of an array.
        ///
        /// The rotation is done with the Reversal Algorithm (John Bentley's
        /// "Programming Pearls").
        fn rotate(&mut self,
                  start: usize, len: usize, shift: usize, stride: usize)
        {
            let pivot = start + shift*stride;

            assert!(len > shift);
            reverse(&mut self.pixels, start, len,       stride);
            reverse(&mut self.pixels, start, shift,     stride);
            reverse(&mut self.pixels, pivot, len-shift, stride);
        }
    }

    // {{{ Display

    impl fmt::Display for Screen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0 .. self.height {
                for j in 0 .. self.width {
                    let idx = indices_to_index(i, j, self.width);
                    write!(f, "{}",
                           if self.pixels[idx] {
                               "#"
                           } else {
                               "."
                           })?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    // }}}
    // {{{ Helpers

    /// Converts 2D indices into a 1D index.
    #[inline(always)]
    fn indices_to_index(line: u16, column: u16, width: u16) -> usize {
        (line*width + column) as usize
    }

    /// Reverse an array or a part of an array.
    fn reverse<T>(arr: &mut [T], begin: usize, len: usize, stride: usize) {
        let mut begin = begin;
        let mut end   = begin + (len - 1)*stride;

        for _ in 0 .. len/2 {
            arr.swap(begin, end);
            begin += stride;
            end   -= stride;
        }
    }

    // }}}
    // {{{ Tests

    #[test]
    fn rotate_whole_array_even() {
        let mut arr  = vec![0, 1, 2, 3];
        let expected = vec![3, 2, 1, 0];

        let len = arr.len();
        reverse(&mut arr, 0, len, 1);

        assert_eq!(arr, expected);
    }

    #[test]
    fn rotate_whole_array_odd() {
        let mut arr  = vec![0, 1, 2];
        let expected = vec![2, 1, 0];

        let len = arr.len();
        reverse(&mut arr, 0, len, 1);

        assert_eq!(arr, expected);
    }

    #[test]
    fn rotate_single_row_even() {
        let row_len  = 4;
        let mut arr  = vec![0, 1,  2,  3,
                            4, 5,  6,  7,
                            8, 9, 10, 11];
        let expected = vec![0, 1,  2,  3,
                            7, 6,  5,  4,
                            8, 9, 10, 11];

        reverse(&mut arr, 1*row_len, row_len, 1);
        assert_eq!(arr, expected);
    }

    #[test]
    fn rotate_single_row_odd() {
        let row_len  = 3;
        let mut arr  = vec![0, 1, 2,
                            3, 4, 5,
                            6, 7, 8];
        let expected = vec![0, 1, 2,
                            5, 4, 3,
                            6, 7, 8];
        reverse(&mut arr, 1*row_len, row_len, 1);
        assert_eq!(arr, expected);
    }

    #[test]
    fn rotate_single_col_even() {
        let row_len  = 4;
        let col_len  = 4;
        let mut arr  = vec![ 0,  1,  2,  3,
                             4,  5,  6,  7,
                             8,  9, 10, 11,
                            12, 13, 14, 15];
        let expected = vec![ 0,  1, 14,  3,
                             4,  5, 10,  7,
                             8,  9,  6, 11,
                            12, 13,  2, 15];

        reverse(&mut arr, 2, col_len, row_len);
        assert_eq!(arr, expected);
    }

    #[test]
    fn rotate_single_col_odd() {
        let row_len  = 4;
        let col_len  = 3;
        let mut arr  = vec![ 0,  1,  2,  3,
                             4,  5,  6,  7,
                             8,  9, 10, 11];
        let expected = vec![ 0,  9,  2,  3,
                             4,  5,  6,  7,
                             8,  1, 10, 11];

        reverse(&mut arr, 1, col_len, row_len);
        assert_eq!(arr, expected);
    }

    // }}}
}

// }}}
// {{{ Instruction

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    TurnOn       { width: u16, height: u16 },
    RotateRow    { row: usize, shift: usize},
    RotateColumn { col: usize, shift: usize},
}

/// Parse the instruction to turn on a rectangle of pixels.
/// The input string should match one the regexp: ^rect \d+x\d+$
named!(turn_on<&[u8], (u16, u16)>,
    do_parse!(
       tag!("rect ")                                >>
       width: map_res!(digit, std::str::from_utf8)  >>
       tag!("x")                                    >>
       height: map_res!(digit, std::str::from_utf8) >>
       (width.parse::<u16>().unwrap(), height.parse::<u16>().unwrap())
   )
);

/// Parse the instruction to rotate a row.
/// The input string should match one the regexp: ^rotate row y=\d+ by \d+$
named!(rotate_row<&[u8], (usize, usize)>,
    do_parse!(
       tag!("rotate row y=")                       >>
       row: map_res!(digit, std::str::from_utf8)   >>
       tag!(" by ")                                >>
       shift: map_res!(digit, std::str::from_utf8) >>
       (row.parse::<usize>().unwrap(), shift.parse::<usize>().unwrap())
    )
);

/// Parse the instruction to rotate a column.
/// The input string should match one the regexp: ^rotate column x=\d+ by \d+$
named!(rotate_col<&[u8], (usize, usize)>,
    do_parse!(
       tag!("rotate column x=")                    >>
       col: map_res!(digit, std::str::from_utf8)   >>
       tag!(" by ")                                >>
       shift: map_res!(digit, std::str::from_utf8) >>
       (col.parse::<usize>().unwrap(), shift.parse::<usize>().unwrap())
    )
);

/// Parse an instruction.
named!(parse_instruction<&[u8], Instruction>,
    alt!(
        turn_on    => {|(w, h)| Instruction::TurnOn { width: w, height: h }}
      | rotate_row => {|(r, s)| Instruction::RotateRow    { row: r, shift: s }}
      | rotate_col => {|(c, s)| Instruction::RotateColumn { col: c, shift: s }}
    )
);

// }}}

use screen::Screen;

pub fn execute(screen : &mut Screen, instructions: &[Instruction]) {
    for instruction in instructions.iter() {
        match instruction {
            &Instruction::TurnOn { width, height }
                => screen.turn_on(width, height),
            &Instruction::RotateRow { row, shift }
                => screen.rotate_row(row, shift),
            &Instruction::RotateColumn { col, shift }
                => screen.rotate_col(col, shift),
        }
    }
}

fn main() {
    let mut file   = File::open("input.txt").unwrap();
    let mut input  = String::new();

    file.read_to_string(&mut input).unwrap();
    let instructions = input.lines()
                            .map(|s| parse_instruction(s.as_bytes()).to_result()
                                                                    .unwrap())
                            .collect::<Vec<_>>();
    let mut screen = Screen::new(50, 6);
    execute(&mut screen, &instructions);
    println!("After swiping the card, there are {} pixels lit on the screen.",
             screen.pixels_lit());
    println!("{}", screen);
}

// {{{ Tests

#[test]
fn examples_part1() {
    let mut screen = Screen::new(7, 3);

    screen.turn_on(3, 2);
    screen.rotate_col(1, 1);
    screen.rotate_row(0, 4);
    screen.rotate_col(1, 1);

    assert_eq!(screen.pixels_lit(), 6);
}

// }}}
