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

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

// }}}
// {{{ Room

/// Represents a room, with its encrypted name and a sector ID.
#[derive(Debug)]
struct Room {
    name: String,
    sector: i32,
}

impl FromStr for Room {
    type Err = String;

    /// Try to build a Room from a string formatted as "name-sector[checksum]"
    ///
    /// The parsing will fail if the name doesn't match the checksum.
    fn from_str(s: &str) -> Result<Room, String> {
        // Extract the encrypted name
        // Extract until we reach a digit (the start of the sector ID).
        let name : String = s.chars().take_while(|c| !c.is_digit(10)).collect();
        // Extract the sector ID
        // Skip the name and extract until '[' (start of the checksum).
        let mut chars = s.chars();
        let sector : i32 = chars.by_ref()
                                .skip(name.len())
                                .take_while(|c| *c != '[')
                                .collect::<String>().parse().unwrap();
        // Extract the checksum
        // From the current position, extract until ']' (end of the checksum).
        let checksum : String = chars.by_ref()
                                     .take_while(|c| c.is_alphabetic())
                                     .collect();
        // Ensure that we are at the end of string.
        assert!(chars.next() == None);
        // Check if the room is legit.
        Room::new(&name, sector, &checksum)
             .ok_or("the room is a decoy".into())
    }
}

impl Room {
    /// Initializes a room
    ///
    /// The checksum is verified, if it doesn't match the encrypted name the
    /// initialization fails.
    fn new(name: &str, sector: i32, checksum: &str) -> Option<Room> {
        // Compute letter frequencies.
        let mut ch_freq = HashMap::new();
        for c in name.chars().filter(|c| *c != '-') {
            let count = ch_freq.entry(c).or_insert(0);
            *count += 1;
        }
        // Sort by:
        // - descending frequency.
        // - ascending alphabetical order if the frequencies are equal.
        let mut sorted_chars : Vec<(char, i32)> = ch_freq.into_iter().collect();
        sorted_chars.sort_by(|&(ch1, cnt1), &(ch2, cnt2)| {
            match cnt2.cmp(&cnt1) {
                Ordering::Equal => ch1.cmp(&ch2),
                res             => res,
            }
        });
        // Compute and verify the checksum.
        let current_checksum = sorted_chars.iter().take(checksum.len())
                                           .map(|&(ch, _)| ch)
                                           .collect::<String>();
        if current_checksum == checksum {
            return Some( Room { name: name.to_owned(), sector: sector })
        }
        None
    }
}

// }}}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let sum = input.lines().filter_map(|s| s.parse::<Room>().ok())
                           .fold(0, |acc, room| acc + room.sector);
    println!("{}", sum);
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert!("aaaaa-bbb-z-y-x-123[abxyz]"  .parse::<Room>().is_ok());
    assert!("a-b-c-d-e-f-g-h-987[abcde]"  .parse::<Room>().is_ok());
    assert!("not-a-real-room-404[oarel]"  .parse::<Room>().is_ok());
    assert!("totally-real-room-200[decoy]".parse::<Room>().is_err());
}

// }}}
