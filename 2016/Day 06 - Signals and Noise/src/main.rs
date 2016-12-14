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
use std::fs::File;
use std::io::Read;

// }}}
// {{{ Consensus computation

/// Computes the original message from a list of corrupted messages.
///
/// XXX: every messages should have the same length.
///
/// For each letter, the chosen one is the most common letter among all the
/// corrupted messages.
fn compute_consensus_most_common(messages: &[&str]) -> String {
    compute_consensus(messages, 0, |(cur_ch, cur_cnt), (ch, cnt)| {
        if cnt > &cur_cnt {
            (*ch, *cnt)
        } else {
            (cur_ch, cur_cnt)
        }
    })
}

/// Computes the original message from a list of corrupted messages.
///
/// XXX: every messages should have the same length.
///
/// For each letter, the chosen one is the least common letter among all the
/// corrupted messages.
fn compute_consensus_least_common(messages: &[&str]) -> String {
    compute_consensus(messages, std::i32::MAX, |(cur_ch, cur_cnt), (ch, cnt)| {
        if cnt <= &cur_cnt {
            (*ch, *cnt)
        } else {
            (cur_ch, cur_cnt)
        }
    })
}

/// Computes the original message from a list of corrupted messages.
///
/// XXX: every messages should have the same length.
///
/// For each letter, the best candidate is chosen among all the corrupted
/// messages according to the function `select`.
fn compute_consensus<F>(messages: &[&str], init: i32, select: F) -> String
    where F: Fn((char, i32), (&char, &i32)) -> (char, i32)
{
    let msg_len = messages[0].len();
    let mut message = String::with_capacity(msg_len);

    let mut ch_freq = HashMap::new();
    for ch_idx in 0..msg_len {
        // Compute letter frequencies.
        for msg_idx in 0..messages.len() {
            let ch = messages[msg_idx].as_bytes()[ch_idx] as char;
            *ch_freq.entry(ch).or_insert(0) += 1;
        }
        // Find the letter that match the criterion.
        let (ch, _) = ch_freq.iter().fold((' ', init), &select);
        message.push(ch);

        ch_freq.clear();
    }
    message
}

// }}}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let messages = input.lines().collect::<Vec<_>>();

    println!("The error-corrected message (simple repetition code) is {:?}.",
             compute_consensus_most_common(&messages));
    println!("The error-corrected message (modified repetition code) is {:?}.",
             compute_consensus_least_common(&messages));
}

// {{{ Tests

#[test]
fn examples() {
    let messages = [
        "eedadn",
        "drvtee",
        "eandsr",
        "raavrd",
        "atevrs",
        "tsrnev",
        "sdttsa",
        "rasrtv",
        "nssdts",
        "ntnada",
        "svetve",
        "tesnvt",
        "vntsnd",
        "vrdear",
        "dvrsen",
        "enarar",
    ];
    assert_eq!(compute_consensus_most_common(&messages),  "easter");
    assert_eq!(compute_consensus_least_common(&messages), "advent");
}

// }}}
