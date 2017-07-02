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

extern crate byteorder;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// }}}
// {{{ MD5

mod md5 {
    use std::io::Cursor;
    use std::mem;
    use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

    pub fn digest(bytes : &[u8]) -> [u8; 16] {
        let mut digest : [u8; 16] = unsafe { mem::uninitialized() };
        let mut buf               = bytes.to_vec();

        // {{{ 1. Padding

        buf.push(0x80u8);
        if buf.len()%64 > 56 {
            let buf_len = buf.len();
            buf.extend(vec![0; 64 - buf_len%64].iter());
        }
        let buf_len = buf.len();
        buf.extend(vec![0; 56 - buf_len%64].iter());

        // }}}
        // {{{ 2. Append length

        let lo : u32 = ((bytes.len() * 8) & 0x0000FFFF) as u32;
        let hi : u32 = ((bytes.len() * 8) >> 32)        as u32;
        buf.write_u32::<LittleEndian>(lo).unwrap();
        buf.write_u32::<LittleEndian>(hi).unwrap();
        let buf_len = buf.len() as u64;
        assert_eq!(buf_len % 64, 0,
                   "message should be an exact multiple of 512 bits");

        // }}}
        // {{{ 3. Initialize MD Buffer

        let mut state = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

        // }}}
        // {{{ 4. Process Message in 16-Word Blocks

        let mut rdr = Cursor::new(buf);
        // Process each 16-word block.
        while rdr.position() < buf_len {
            let mut x : [u32; 16] = unsafe { mem::uninitialized() };
            // Copy current block into X.
            for j in 0..16 {
                x[j] = rdr.read_u32::<LittleEndian>().unwrap();
            }

            let mut a = state[0];
            let mut b = state[1];
            let mut c = state[2];
            let mut d = state[3];
            // {{{ Round 1

            a = apply(function_f, a, b, c, d, x[ 0],  7, 0xd76aa478); // 1
            d = apply(function_f, d, a, b, c, x[ 1], 12, 0xe8c7b756); // 2
            c = apply(function_f, c, d, a, b, x[ 2], 17, 0x242070db); // 3
            b = apply(function_f, b, c, d, a, x[ 3], 22, 0xc1bdceee); // 4
            a = apply(function_f, a, b, c, d, x[ 4],  7, 0xf57c0faf); // 5
            d = apply(function_f, d, a, b, c, x[ 5], 12, 0x4787c62a); // 6
            c = apply(function_f, c, d, a, b, x[ 6], 17, 0xa8304613); // 7
            b = apply(function_f, b, c, d, a, x[ 7], 22, 0xfd469501); // 8
            a = apply(function_f, a, b, c, d, x[ 8],  7, 0x698098d8); // 9
            d = apply(function_f, d, a, b, c, x[ 9], 12, 0x8b44f7af); // 10
            c = apply(function_f, c, d, a, b, x[10], 17, 0xffff5bb1); // 11
            b = apply(function_f, b, c, d, a, x[11], 22, 0x895cd7be); // 12
            a = apply(function_f, a, b, c, d, x[12],  7, 0x6b901122); // 13
            d = apply(function_f, d, a, b, c, x[13], 12, 0xfd987193); // 14
            c = apply(function_f, c, d, a, b, x[14], 17, 0xa679438e); // 15
            b = apply(function_f, b, c, d, a, x[15], 22, 0x49b40821); // 16

            // }}}
            // {{{ Round 2

            a = apply(function_g, a, b, c, d, x[ 1],  5, 0xf61e2562); // 17
            d = apply(function_g, d, a, b, c, x[ 6],  9, 0xc040b340); // 18
            c = apply(function_g, c, d, a, b, x[11], 14, 0x265e5a51); // 19
            b = apply(function_g, b, c, d, a, x[ 0], 20, 0xe9b6c7aa); // 20
            a = apply(function_g, a, b, c, d, x[ 5],  5, 0xd62f105d); // 21
            d = apply(function_g, d, a, b, c, x[10],  9,  0x2441453); // 22
            c = apply(function_g, c, d, a, b, x[15], 14, 0xd8a1e681); // 23
            b = apply(function_g, b, c, d, a, x[ 4], 20, 0xe7d3fbc8); // 24
            a = apply(function_g, a, b, c, d, x[ 9],  5, 0x21e1cde6); // 25
            d = apply(function_g, d, a, b, c, x[14],  9, 0xc33707d6); // 26
            c = apply(function_g, c, d, a, b, x[ 3], 14, 0xf4d50d87); // 27
            b = apply(function_g, b, c, d, a, x[ 8], 20, 0x455a14ed); // 28
            a = apply(function_g, a, b, c, d, x[13],  5, 0xa9e3e905); // 29
            d = apply(function_g, d, a, b, c, x[ 2],  9, 0xfcefa3f8); // 30
            c = apply(function_g, c, d, a, b, x[ 7], 14, 0x676f02d9); // 31
            b = apply(function_g, b, c, d, a, x[12], 20, 0x8d2a4c8a); // 32

            // }}}
            // {{{ Round 3

            a = apply(function_h, a, b, c, d, x[ 5],  4, 0xfffa3942); // 33
            d = apply(function_h, d, a, b, c, x[ 8], 11, 0x8771f681); // 34
            c = apply(function_h, c, d, a, b, x[11], 16, 0x6d9d6122); // 35
            b = apply(function_h, b, c, d, a, x[14], 23, 0xfde5380c); // 36
            a = apply(function_h, a, b, c, d, x[ 1],  4, 0xa4beea44); // 37
            d = apply(function_h, d, a, b, c, x[ 4], 11, 0x4bdecfa9); // 38
            c = apply(function_h, c, d, a, b, x[ 7], 16, 0xf6bb4b60); // 39
            b = apply(function_h, b, c, d, a, x[10], 23, 0xbebfbc70); // 40
            a = apply(function_h, a, b, c, d, x[13],  4, 0x289b7ec6); // 41
            d = apply(function_h, d, a, b, c, x[ 0], 11, 0xeaa127fa); // 42
            c = apply(function_h, c, d, a, b, x[ 3], 16, 0xd4ef3085); // 43
            b = apply(function_h, b, c, d, a, x[ 6], 23,  0x4881d05); // 44
            a = apply(function_h, a, b, c, d, x[ 9],  4, 0xd9d4d039); // 45
            d = apply(function_h, d, a, b, c, x[12], 11, 0xe6db99e5); // 46
            c = apply(function_h, c, d, a, b, x[15], 16, 0x1fa27cf8); // 47
            b = apply(function_h, b, c, d, a, x[ 2], 23, 0xc4ac5665); // 48

            // }}}
            // {{{ Round 4

            a = apply(function_i, a, b, c, d, x[ 0],  6, 0xf4292244); // 49
            d = apply(function_i, d, a, b, c, x[ 7], 10, 0x432aff97); // 50
            c = apply(function_i, c, d, a, b, x[14], 15, 0xab9423a7); // 51
            b = apply(function_i, b, c, d, a, x[ 5], 21, 0xfc93a039); // 52
            a = apply(function_i, a, b, c, d, x[12],  6, 0x655b59c3); // 53
            d = apply(function_i, d, a, b, c, x[ 3], 10, 0x8f0ccc92); // 54
            c = apply(function_i, c, d, a, b, x[10], 15, 0xffeff47d); // 55
            b = apply(function_i, b, c, d, a, x[ 1], 21, 0x85845dd1); // 56
            a = apply(function_i, a, b, c, d, x[ 8],  6, 0x6fa87e4f); // 57
            d = apply(function_i, d, a, b, c, x[15], 10, 0xfe2ce6e0); // 58
            c = apply(function_i, c, d, a, b, x[ 6], 15, 0xa3014314); // 59
            b = apply(function_i, b, c, d, a, x[13], 21, 0x4e0811a1); // 60
            a = apply(function_i, a, b, c, d, x[ 4],  6, 0xf7537e82); // 61
            d = apply(function_i, d, a, b, c, x[11], 10, 0xbd3af235); // 62
            c = apply(function_i, c, d, a, b, x[ 2], 15, 0x2ad7d2bb); // 63
            b = apply(function_i, b, c, d, a, x[ 9], 21, 0xeb86d391); // 64

            // }}}
            state[0] = state[0].wrapping_add(a);
            state[1] = state[1].wrapping_add(b);
            state[2] = state[2].wrapping_add(c);
            state[3] = state[3].wrapping_add(d);
        }

        // }}}
        // {{{ 5. Output

        let mut buf = Vec::with_capacity(16);

        buf.write_u32::<LittleEndian>(state[0]).unwrap();
        buf.write_u32::<LittleEndian>(state[1]).unwrap();
        buf.write_u32::<LittleEndian>(state[2]).unwrap();
        buf.write_u32::<LittleEndian>(state[3]).unwrap();

        assert_eq!(buf.len(), 16);

        digest.clone_from_slice(&buf);

        // }}}

        digest
    }

    #[inline(always)]
    fn function_f(x: u32, y : u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    #[inline(always)]
    fn function_g(x: u32, y : u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    #[inline(always)]
    fn function_h(x: u32, y : u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    #[inline(always)]
    fn function_i(x: u32, y : u32, z: u32) -> u32 {
        y ^ (x | !z)
    }

    #[inline(always)]
    fn apply<F>(fun: F, a: u32, b: u32, c: u32, d: u32, xk: u32, s:u32, ti: u32)
        -> u32
        where F: Fn(u32, u32, u32) -> u32 {
        let mut tmp = a.wrapping_add(fun(b, c, d));
        tmp = tmp.wrapping_add(xk);
        tmp = tmp.wrapping_add(ti);
        b.wrapping_add(tmp.rotate_left(s))
    }
}

// }}}

fn main() {
    let file = File::open("input.txt").expect("cannot open input.txt");
    let mut input  = String::new();

    // All the instructions are on the first line.
    BufReader::new(&file).read_line(&mut input).unwrap();
    assert!(!input.is_empty());
    // Remove trailing new line.
    input.pop();

    let res = md5::digest(input.as_bytes());
    for b in res.iter() {
        print!("{:02x}", b);
    }
    println!("");
}

// {{{ Tests

#[test]
fn test_md5_rfc1321() {
    // See §A.5 in RFC 1321.
    let testcases: &[(&str, &[u8; 16])] = &[
        ("",
         &[0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04,
           0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e]
        ),
        ("a",
         &[0x0c, 0xc1, 0x75, 0xb9, 0xc0, 0xf1, 0xb6, 0xa8,
           0x31, 0xc3, 0x99, 0xe2, 0x69, 0x77, 0x26, 0x61]
        ),
        ("abc",
         &[0x90, 0x01, 0x50, 0x98, 0x3c, 0xd2, 0x4f, 0xb0,
           0xd6, 0x96, 0x3f, 0x7d, 0x28, 0xe1, 0x7f, 0x72]
        ),
        ("message digest",
         &[0xf9, 0x6b, 0x69, 0x7d, 0x7c, 0xb7, 0x93, 0x8d,
           0x52, 0x5a, 0x2f, 0x31, 0xaa, 0xf1, 0x61, 0xd0]
        ),
        ("abcdefghijklmnopqrstuvwxyz",
         &[0xc3, 0xfc, 0xd3, 0xd7, 0x61, 0x92, 0xe4, 0x00,
           0x7d, 0xfb, 0x49, 0x6c, 0xca, 0x67, 0xe1, 0x3b]
        ),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
         &[0xd1, 0x74, 0xab, 0x98, 0xd2, 0x77, 0xd9, 0xf5,
           0xa5, 0x61, 0x1c, 0x2c, 0x9f, 0x41, 0x9d, 0x9f]
        ),
        ("12345678901234567890123456789012345678901234567890123456789012345678901234567890",
         &[0x57, 0xed, 0xf4, 0xa2, 0x2b, 0xe3, 0xc9, 0x55,
           0xac, 0x49, 0xda, 0x2e, 0x21, 0x07, 0xb6, 0x7a]
        )
    ];
    for &(data, expected) in testcases.iter() {
        assert_eq!(md5::digest(data.as_bytes()), *expected);
    }
}

// }}}
