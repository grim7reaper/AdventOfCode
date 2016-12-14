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
use std::io::{self, Write};

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

// }}}

/// Updates the cinematic "decrypting" animation.
fn update_decrypting_animation(door: &str,  password : &[u8]) {
    print!("\rThe password of the {} is {}",
           door, String::from_utf8_lossy(&password));
    io::stdout().flush().unwrap();
}

/// Computes the password from the door ID.
fn decode_password(door: &str, door_id : &[u8], is_in_order: bool) -> String {
    let hexdigit = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                    b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];
    let mut password = [b'_';  8];
    let mut idx = 0;
    let mut md5 = Md5::new();

    update_decrypting_animation(door, &password);
    let _ : Vec<char> = (0u64..).filter_map(|i| {
        let mut hash = [0; 16]; // A MD5 digest is 128 bits (i.e 16 bytes).

        md5.input(door_id);
        md5.input(i.to_string().as_bytes());
        md5.result(&mut hash);
        md5.reset();

        // As one byte == two hex character, we have to tests the 2.5 bytes (two
        // whole bytes + the 4 high bits of the third byte) to see if the digest
        // starts with 5 zeroes.
        if (hash[0] | hash[1] | (hash[2] >> 4)) != 0 {
            return None
        }
        if is_in_order {
            // When the letters are already ordered the letter is the 6th
            // character of the hash
            password[idx] = hexdigit[(hash[2] & 0x0f) as usize];
            idx += 1;
            update_decrypting_animation(door, &password);
            Some('_')
        } else {
            // When the letters are not ordered:
            // - the index is the 6th character of the hash
            // - the letter is the 7th character of the hash
            let pos = hash[2] & 0x0f;
            let ch  = hexdigit[(hash[3] >> 4) as usize];

            if pos < 8 && password[pos as usize] == b'_' {
                password[pos as usize] = ch;
                update_decrypting_animation(door, &password);
                Some('_')
            } else {
                None
            }
        }
    }).take(8).collect();
    println!("");
    String::from_utf8_lossy(&password).to_string()
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    // The Door ID is on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    let door_id = input.trim().as_bytes();
    decode_password("first door",  door_id, true);
    decode_password("second door", door_id, false);
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!(decode_password("abc".as_bytes(), true), "18f47a30");
}

#[test]
fn examples_part2() {
    assert_eq!(decode_password("abc".as_bytes(), false), "05ace8e3");
}

// }}}
