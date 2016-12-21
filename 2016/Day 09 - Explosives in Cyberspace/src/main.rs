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

// }}}
// {{{ Compress

mod compress {
    /// A compression marker.
    #[derive(Debug)]
    struct Marker {
        length: usize,
        repeat: usize,
    }

    /// Parses an usize value from a stream of bytes.
    fn parse_usize<'a, I>(stream: &mut I) -> usize
        where I: Iterator<Item=&'a u8>
    {
        stream.map(|&b| b as char).collect::<String>().parse().unwrap()
    }

    /// Extracts a marker from the stream.
    fn extract_marker<'a, I>(stream: &mut I) -> Marker
        where I: Iterator<Item=&'a u8>
    {
        let mut bytes = stream.take_while(|&b| *b != b')');
        let length = parse_usize(&mut bytes.by_ref()
                                           .take_while(|b| **b != b'x'));
        let repeat = parse_usize(&mut bytes);
        assert!(bytes.next().is_none());

        Marker { length: length, repeat: repeat }
    }

    /// Computes the size of the string after decompression.
    pub fn compute_final_size(s : &str) -> u64 {
        let mut bytes  = s.as_bytes().iter();
        let mut size   = 0;

        while let Some(&b) = bytes.next() {
            if b == b'(' {
                let marker = extract_marker(&mut bytes);
                // Skip `marker.length` bytes.
                bytes.nth(marker.length - 1);
                size += (marker.length*marker.repeat) as u64;
            } else {
                size += 1;
            }
        }
        size
    }
}

// }}}
use compress::compute_final_size;

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    println!("The decompressed length of the file is {} bytes.",
             compute_final_size(&input.trim()));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(compute_final_size("ADVENT"),             6);
    assert_eq!(compute_final_size("A(1x5)BC"),           7);
    assert_eq!(compute_final_size("(3x3)XYZ"),           9);
    assert_eq!(compute_final_size("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(compute_final_size("(6x1)(1x3)A"),        6);
    assert_eq!(compute_final_size("X(8x2)(3x3)ABCY"),   18);
}

// }}}
