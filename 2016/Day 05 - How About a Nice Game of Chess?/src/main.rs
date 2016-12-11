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
use std::io::BufRead;
use std::io::BufReader;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

// }}}

/// Computes the password from the door ID.
///
/// The password is 8 characters long.
fn decode_password(door_id : &str) -> String {
    (0u64..).filter_map(|i| {
        let mut md5 = Md5::new();

        md5.input_str(&format!("{}{}", door_id, i));
        let hash = md5.result_str();

        if hash.starts_with("00000") {
            hash.chars().nth(5)
        } else {
            None
        }
    }).take(8).collect()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = String::new();

    // The Door ID is on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    println!("{}", decode_password(input.trim()));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(decode_password("abc"), "18f47a30");
}

// }}}
