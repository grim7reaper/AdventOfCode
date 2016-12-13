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
use std::str::FromStr;

// }}}
// {{{ IPv7

/// An IPv7 address.
#[derive(Debug)]
struct IPv7 {
    address:   Vec<u8>,
    supernets: Vec<(usize, usize)>,
    hypernets: Vec<(usize, usize)>,
}

impl FromStr for IPv7 {
    type Err = String;

    fn from_str(s: &str) -> Result<IPv7, String> {
        let mut s_net = Vec::new();
        let mut h_net = Vec::new();
        let mut bytes = s.as_bytes().iter();
        let mut begin = 0;     // Start of the current slice.
        let mut end   = 0;     // End of the current slice.
        let mut in_hypernet = false; // Are we in an hypernet sequence?

        while let Some(b) = bytes.next() {
            end += 1;
            if *b == b'[' {
                assert_eq!(in_hypernet, false);
                in_hypernet = true;
                s_net.push((begin, end-1));
                begin = end;
            } else
            if *b == b']' {
                assert_eq!(in_hypernet, true);
                in_hypernet = false;
                h_net.push((begin, end-1));
                begin = end;
            }
        }
        // Check for a trailing supernet.
        if end - begin != 0 {
            s_net.push((begin, end));
        }

        if s_net.is_empty() {
            Err("no supernet sequences".to_owned())
        } else
        if h_net.is_empty() {
            Err("no hypernet sequences".to_owned())
        } else {
            Ok(IPv7 {
                address:   s.to_owned().into_bytes(),
                supernets: s_net,
                hypernets: h_net
            })
        }
    }
}

impl IPv7 {
    /// Test if the IP supports TLS.
    ///
    /// An IP supports TLS if it has an ABBA in its supernets, and no ABBA in
    /// its hypernet sequences.
    fn has_tls_support(&self) -> bool {
        self.supernets.iter().any(|&(a, z)| has_abba(&self.address[a..z])) &&
       !self.hypernets.iter().any(|&(a, z)| has_abba(&self.address[a..z]))
    }
}

// {{{ ABBA helpers

/// Test if `s` is an ABBA.
///
/// An ABBA is any four-character sequence which consists of a pair of two
/// different characters followed by the reverse of that pair, such as xyyx or
/// `abba`.
pub fn is_abba(s: &[u8]) -> bool {
    s.len() == 4    &&
    s[0]    == s[3] &&
    s[1]    == s[2] &&
    s[0]    != s[1]
}

/// Test if `s` contains an ABBA.
pub fn has_abba(s: &[u8]) -> bool {
    s.windows(4).any(is_abba)
}

// }}}
// }}}

fn main() {
    let mut file  = File::open("input.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let ips = input.lines()
                   .filter_map(|ip| ip.parse::<IPv7>().ok())
                   .collect::<Vec<_>>();
    println!("{}", ips.iter().filter(|ip| ip.has_tls_support()).count());
}

// {{{ Tests

#[test]
fn examples_part1() {
    assert_eq!("abba[mnop]qrst".parse::<IPv7>().unwrap().has_tls_support(),
               true);
    assert_eq!("abcd[bddb]xyyx".parse::<IPv7>().unwrap().has_tls_support(),
               false);
    assert_eq!("aaaa[qwer]tyui".parse::<IPv7>().unwrap().has_tls_support(),
               false);
    assert_eq!("ioxxoj[asdfgh]zxcvbn".parse::<IPv7>().unwrap().has_tls_support(),
               true);
}

// }}}
