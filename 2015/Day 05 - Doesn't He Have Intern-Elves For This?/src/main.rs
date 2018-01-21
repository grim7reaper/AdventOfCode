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

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::Read;

// }}}

fn is_nice_string(s: &str) -> bool {
    let mut has_pair = false;
    for pair in s.as_bytes().windows(2) {
        // Check for ab, cd, pq, or xy.
        if pair == b"ab" || pair == b"cd" || pair == b"pq" || pair == b"xy" {
            return false;
        }
        // Search one letter that appears twice in a row.
        if pair[0] == pair[1] {
            has_pair = true;
        }
    }
    // Check if the string contains at least three vowels.
    let mut vowels = 0;
    for c in s.as_bytes() {
        vowels += match *c {
            b'a' | b'e' | b'i' | b'o' | b'u' => 1,
            _ => 0
        };
        if vowels == 3 {
            return has_pair;
        }
    }
    false
}

fn is_nice_string_v2(s: &str) -> bool {
    let mut has_pair = false;
    // Search a pair of any two letters that appears at least twice.
    let mut pairs = HashMap::new();
    for (i, pair) in s.as_bytes().windows(2).enumerate() {
        match pairs.entry(pair) {
            // First time we see this pair: insert the position.
            Entry::Vacant(e)   => { e.insert(i); () }
            // We know this pair, check for overlap!
            Entry::Occupied(e) => if i - e.get() > 1 {
                has_pair = true;
                break;
            }
        };
    }
    // Search one letter which repeats with exactly one letter between them.
    for triple in s.as_bytes().windows(3) {
        if triple[0] == triple[2] {
            return has_pair;
        }
    }
    false
}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    println!("There are {} nice strings.",
             input.lines().filter(|&s| is_nice_string(s)).count());
    println!("There are {} truly nice strings.",
             input.lines().filter(|&s| is_nice_string_v2(s)).count());
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(is_nice_string("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice_string("aaa"),              true);
    assert_eq!(is_nice_string("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice_string("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice_string("dvszwmarrgswjxmb"), false);
}

#[test]
fn examples_part2() {
    assert_eq!(is_nice_string_v2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_nice_string_v2("xxyxx"),            true);
    assert_eq!(is_nice_string_v2("aaaa"),             true);
    assert_eq!(is_nice_string_v2("aaa"),              false);
    assert_eq!(is_nice_string_v2("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice_string_v2("ieodomkazucvgmuy"), false);
}

// }}}
