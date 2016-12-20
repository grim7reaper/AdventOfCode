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

    /// Expands a marker.
    ///
    /// `length` bytes are extracted from the stream and they are repeated `repeat`
    /// times.
    fn expand_marker<'a, I>(stream: &mut I, marker: Marker) -> Vec<u8>
        where I: Iterator<Item=&'a u8>
    {
        let payload = stream.take(marker.length).collect::<Vec<_>>();
        assert_eq!(payload.len(), marker.length);

        payload.into_iter().cycle().take(marker.repeat*marker.length)
               .map(|b| *b).collect::<Vec<_>>()
    }

    /// Decompresses the input string.
    pub fn decompress(s : &str) -> String {
        let mut bytes  = s.as_bytes().iter();
        let mut result = Vec::with_capacity(s.len());

        while let Some(&b) = bytes.next() {
            if b == b'(' {
                let marker = extract_marker(&mut bytes);
                result.extend(expand_marker(&mut bytes, marker));
            } else {
                result.push(b);
            }
        }
        String::from_utf8_lossy(&result).into_owned()
    }
}

// }}}
use compress::decompress;

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    println!("The decompressed length of the file is {}.",
             decompress(&input.trim()).len());
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(decompress("ADVENT"),            "ADVENT");
    assert_eq!(decompress("A(1x5)BC"),          "ABBBBBC");
    assert_eq!(decompress("(3x3)XYZ"),          "XYZXYZXYZ");
    assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
    assert_eq!(decompress("(6x1)(1x3)A"),       "(1x3)A");
    assert_eq!(decompress("X(8x2)(3x3)ABCY"),   "X(3x3)ABC(3x3)ABCY");
}

// }}}
