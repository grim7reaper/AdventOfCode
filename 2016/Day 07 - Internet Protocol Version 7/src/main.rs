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
#[cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
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
        let mut begin = 0;     // Start of the current slice.
        let mut end   = 0;     // End of the current slice.
        let mut in_hypernet = false; // Are we in an hypernet sequence?

        for b in s.as_bytes() {
            end += 1;
            if *b == b'[' {
                assert_eq!(in_hypernet, false);
                in_hypernet = true;
                s_net.push((begin, end-1));
                begin = end;
            } else if *b == b']' {
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
        } else if h_net.is_empty() {
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

    /// Test if the IP supports SSL.
    ///
    /// An IP supports SSL if it has an ABA, anywhere in the supernet sequences,
    /// and a corresponding BAB, anywhere in the hypernet sequences.
    fn has_ssl_support(&self) -> bool {
        let abas = self.supernets.iter().fold(Vec::new(), |mut acc, &(a, z)| {
            acc.extend(collect_aba(&self.address[a..z]));
            acc
        });
        abas.iter().any(|aba| {
            self.hypernets.iter().any(|&(a, z)| {
                has_matching_bab_for(&self.address[a..z], aba)
            })
        })
    }
}

// {{{ ABBA helpers

/// Test if `s` is an ABBA (Autonomous Bridge Bypass Annotation).
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
// {{{ ABA/BAB helpers

/// Test if `s` is an ABA (Area-Broadcast Accessor).
///
/// An ABA is any three-character sequence which consists of the same character
/// twice with a different character between them, such as `xyx` or `aba`.
pub fn is_aba(s: &[u8]) -> bool {
    s.len() == 3    &&
    s[0]    == s[2] &&
    s[0]    != s[1]
}

/// Returns a list of ABAs contained in `s`.
pub fn collect_aba(s: &[u8]) -> Vec<&[u8]> {
    s.windows(3).filter(|candidate| is_aba(candidate)).collect()
}

/// Test if `s` contains an BAB matching the given ABA.
pub fn has_matching_bab_for(s: &[u8], aba: &[u8]) -> bool {
    assert!(is_aba(aba));
    let bab = [ aba[1], aba[0], aba[1] ];
    s.windows(3).any(|candidate| candidate == bab)
}

// }}}
// }}}

fn main() {
    let mut file  = File::open("input.txt").expect("cannot open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let ips = input.lines()
                   .filter_map(|ip| ip.parse::<IPv7>().ok())
                   .collect::<Vec<_>>();
    println!("There are {} IPs with TLS support.",
             ips.iter().filter(|ip| ip.has_tls_support()).count());
    println!("There are {} IPs with SSL support.",
             ips.iter().filter(|ip| ip.has_ssl_support()).count());
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

#[test]
fn examples_part2() {
    assert_eq!("aba[bab]xyz".parse::<IPv7>().unwrap().has_ssl_support(),   true);
    assert_eq!("xyx[xyx]xyx".parse::<IPv7>().unwrap().has_ssl_support(),   false);
    assert_eq!("aaa[kek]eke".parse::<IPv7>().unwrap().has_ssl_support(),   true);
    assert_eq!("zazbz[bzb]cdb".parse::<IPv7>().unwrap().has_ssl_support(), true);
}

// }}}
