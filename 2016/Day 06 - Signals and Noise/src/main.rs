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


/// Computes the original message from a list of corrupted messages.
///
/// XXX: every messages should have the same length.
///
/// Note that this problem is similar to the consensus sequence in bioinformatic
/// (see http://rosalind.info/glossary/consensus-string/).
fn compute_consensus(messages: &[&str]) -> String {
    let msg_len = messages[0].len();
    let mut message = String::with_capacity(msg_len);

    let mut ch_freq = HashMap::new();
    for ch_idx in 0..msg_len {
        // Compute letter frequencies.
        for msg_idx in 0..messages.len() {
            let ch = messages[msg_idx].as_bytes()[ch_idx] as char;
            *ch_freq.entry(ch).or_insert(0) += 1;
        }
        // Find the most commont letter (iter::max_by is unstable ☹).
        let (ch, _) = ch_freq.iter().fold((' ', 0), |(cur_ch, cur_cnt), (ch, cnt)| {
            if cnt >= &cur_cnt {
                (*ch, *cnt)
            } else {
                (cur_ch, cur_cnt)
            }
        });
        message.push(ch);

        ch_freq.clear();
    }
    message
}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let messages = input.lines().collect::<Vec<_>>();

    println!("{:?}", compute_consensus(&messages));
}

// {{{ Tests

#[test]
fn examples_part1() {
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
    assert_eq!(compute_consensus(&messages), "easter");
}

// }}}
