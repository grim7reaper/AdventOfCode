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
    fn parse_usize(stream: &mut Iterator<Item=&u8>) -> usize {
        stream.map(|&b| b as char).collect::<String>().parse().unwrap()
    }

    /// Extracts a marker from the stream.
    fn extract_marker(stream: &mut Iterator<Item=&u8>) -> Marker {
        let mut bytes = stream.take_while(|&b| *b != b')');
        let length = parse_usize(&mut bytes.by_ref()
                                           .take_while(|&b| *b != b'x'));
        let repeat = parse_usize(&mut bytes);
        assert!(bytes.next().is_none());

        Marker { length: length, repeat: repeat }
    }

    /// Computes the size of the string after decompression.
    fn compute_final_size(mut bytes : &mut Iterator<Item=&u8>,
                          do_recursive_expand: bool) -> u64
    {
        let mut size = 0;

        while let Some(&b) = bytes.next() {
            if b == b'(' {
                let marker   = extract_marker(&mut bytes);
                let mut data = bytes.take(marker.length);
                let len = if do_recursive_expand {
                    compute_final_size(&mut data, true)
                } else {
                    // Consumes the iterator.
                    let _ = data.last();
                    marker.length as u64
                };
                size += len * (marker.repeat as u64);
            } else {
                size += 1;
            }
        }
        size
    }

    /// Computes the size of the string after decompression (algorithm v1).
    pub fn compute_final_size_v1(s : &str) -> u64 {
        compute_final_size(&mut s.as_bytes().iter(), false)
    }

    /// Computes the size of the string after decompression (algorithm v2).
    ///
    /// The v2 of the algorithm use recursive expansion of the markers.
    pub fn compute_final_size_v2(s : &str) -> u64 {
        compute_final_size(&mut s.as_bytes().iter(), true)
    }
}

// }}}
use compress::{compute_final_size_v1,compute_final_size_v2};

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let input = input.trim();
    println!("The decompressed length of the file is {} bytes.",
             compute_final_size_v1(input));
    println!("The real decompressed length of the file is {} bytes.",
             compute_final_size_v2(input));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(compute_final_size_v1("ADVENT"),             6);
    assert_eq!(compute_final_size_v1("A(1x5)BC"),           7);
    assert_eq!(compute_final_size_v1("(3x3)XYZ"),           9);
    assert_eq!(compute_final_size_v1("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(compute_final_size_v1("(6x1)(1x3)A"),        6);
    assert_eq!(compute_final_size_v1("X(8x2)(3x3)ABCY"),   18);
}

#[test]
fn examples_part2() {
    assert_eq!(compute_final_size_v2("(3x3)XYZ"),           9);
    assert_eq!(compute_final_size_v2("X(8x2)(3x3)ABCY"),   20);
    assert_eq!(compute_final_size_v2(
            "(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(compute_final_size_v2(
            "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
}

// }}}
