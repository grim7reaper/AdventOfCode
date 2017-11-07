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

extern crate regex;
use regex::Regex;

// }}}

// Return he number of characters in the code representation of the string.
fn code_length(s: &str) -> usize {
    s.len()
}

// Return the number of characters in the in-memory string.
fn memory_length(s: &str) -> usize {
    // -2 because we ignore the surrounding double quotes.
    let len = code_length(s) - 2;

    let esc = Regex::new(r#"\\\\|\\""#).unwrap();
    // -1 per \\ and \" because they occupies two characters but count as one.
    let simple_esc = esc.find_iter(s).count();

    // -3 per \xHH because they uses four characters but count as one.
    let hex = Regex::new(r"(?P<prefix>\\+)x[[:xdigit:]]{2}").unwrap();
    let hex_esc = hex.captures_iter(s).fold(0, |acc, caps|
        // Only count \xFF, not \\xFF.
        if caps["prefix"].len() % 2 == 1 {
            acc + 3
        } else {
            acc
        }
    );
    len - simple_esc - hex_esc
}

// Return the number of characters in the encoded string.
fn encoded_length(s: &str) -> usize {
    let len = code_length(s) + 2;

    let esc = Regex::new(r#"\\|""#).unwrap();
    len + esc.find_iter(s).count()
}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let code_sum: usize = input.lines().map(|l| code_length(l)).sum();
    let mem_sum:  usize = input.lines().map(|l| memory_length(l)).sum();
    let enc_sum:  usize = input.lines().map(|l| encoded_length(l)).sum();

    println!("The difference between code and in-memory is {}",
             code_sum - mem_sum);
    println!("The difference between encoded and code is {}",
             enc_sum - code_sum);
}

// {{{ Tests

#[test]
fn examples_part1() {
    let input = vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];

    let code_sum: usize = input.iter().map(|s| code_length(s)).sum();
    let mem_sum:  usize = input.iter().map(|s| memory_length(s)).sum();

    assert_eq!(code_sum - mem_sum, 12);
}

#[test]
fn examples_part2() {
    let input = vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];

    let code_sum: usize = input.iter().map(|s| code_length(s)).sum();
    let enc_sum:  usize = input.iter().map(|s| encoded_length(s)).sum();

    assert_eq!(enc_sum - code_sum, 19);
}

// }}}
