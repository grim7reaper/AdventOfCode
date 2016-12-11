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
fn decode_password(door_id : &str, is_in_order: bool) -> String {
    let mut has_letter = [false; 8];
    let mut letters : Vec<(u32, char)> = (0u64..).filter_map(|i| {
        let mut md5 = Md5::new();

        md5.input_str(&format!("{}{}", door_id, i));
        let hash = md5.result_str();

        if !hash.starts_with("00000") {
            return None
        }
        if is_in_order {
            // When the letters are already ordered:
            // - the letter is the 6th character of the hash
            // - we can use 0 as position (it will be ignored anyway).
            Some((0, hash.chars().nth(5).unwrap()))
        } else {
            // When the letters are not ordered:
            // - the index is the 6th character of the hash
            // - the letter is the 7th character of the hash
            let pos = hash.chars().nth(5).unwrap().to_digit(10).unwrap_or(10);
            let ch  = hash.chars().nth(6).unwrap();

            if pos < 8 && !has_letter[pos as usize] {
                has_letter[pos as usize] = true;
                Some((pos, ch))
            } else {
                None
            }
        }
    }).take(8).collect();
    if !is_in_order {
        letters.sort();
    }
    letters.iter().map(|&(_, ch)| ch).collect()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = String::new();

    // The Door ID is on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    println!("{}", decode_password(input.trim(), true));
    println!("{}", decode_password(input.trim(), false));
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(decode_password("abc", true), "18f47a30");
}

#[test]
fn examples_part2() {
    assert_eq!(decode_password("abc", false), "05ace8e3");
}

// }}}
