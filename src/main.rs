//! CISC 260 - HW One - Part One
//! Integer to Hexadecimal and Binary

use itertools::Itertools;
use std::collections::HashMap;
use std::io;

fn main() {
    let input = get_input();
    let fmt_bin = convert_binary(&input);
    let fmt_hex = convert_hex(&fmt_bin);

    println!("Bin, Computed:\t{fmt_bin}, ({} digits)", fmt_bin.len());
    println!("Bin, Actual:\t{input:032b}");
    println!("Hex, Computed:\t{fmt_hex}, ({} digits)", fmt_hex.len());
    println!("Hex, Actual:\t{input:08X?}");
}

/// Returns a formatted binary `String`, given an i32.
///
/// Note: `int` cannot be `i32::MIN` due to `abs()` overflow,
/// See https://doc.rust-lang.org/std/primitive.i32.html#method.abs
fn convert_binary(int: &i32) -> String {
    let raw_bin = pos_int_to_bin(&int.abs()).unwrap();
    let pfmt_bin = pos_fmt_bin(&raw_bin);

    match int.is_negative() {
        false => pfmt_bin,
        true => {
            let flip_bin = two_comp_flip_bin(&pfmt_bin);
            two_comp_add_one_bin(&flip_bin)
        }
    }
}

/// Returns an 8 digit hexadecimal by parsing the formatted binary
/// `str` in chunks of 4. Each chunk is translated to a hex value
/// via a hashmap.
fn convert_hex(fmt_bin: &str) -> String {
    let hex_map = HashMap::from([
        ("0000", "0"),
        ("0001", "1"),
        ("0010", "2"),
        ("0011", "3"),
        ("0100", "4"),
        ("0101", "5"),
        ("0110", "6"),
        ("0111", "7"),
        ("1000", "8"),
        ("1001", "9"),
        ("1010", "A"),
        ("1011", "B"),
        ("1100", "C"),
        ("1101", "D"),
        ("1110", "E"),
        ("1111", "F"),
    ]);

    fmt_bin
        .chars()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            hex_map
                .get(chunk.collect::<String>().as_str())
                .unwrap()
                .to_string()
        })
        .collect()
}

fn get_input() -> i32 {
    let mut input = String::new();

    println!("Enter your 32-bit integer: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().parse::<i64>().unwrap() as i32
}

/// Formats a truncated binary `str` to a 32-bit `String` by padding zeros.
fn pos_fmt_bin(raw_bin: &str) -> String {
    format!("{raw_bin:0>32}")
}

/// Converts a positive, 32-bit integer to a binary `String` representation.
/// If the value is negative, return Err.
fn pos_int_to_bin(int: &i32) -> Result<String, &'static str> {
    if int.lt(&0) {
        return Err("Input must be non-negative");
    }

    if int.le(&1) {
        return Ok(int.to_string());
    }

    match int % 2 {
        0 => Ok(pos_int_to_bin(&(int / 2))? + "0"),
        1 => Ok(pos_int_to_bin(&(int / 2))? + "1"),
        _ => unreachable!(),
    }
}

/// Returns a bit-flipped, binary `String`
fn two_comp_flip_bin(raw_bin: &str) -> String {
    raw_bin
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => unreachable!(),
        })
        .collect()
}

/// Returns the binary `String` representation of adding 1 to the input
fn two_comp_add_one_bin(flip_bin: &str) -> String {
    let mut is_added = false;
    let mut pls_bin = String::with_capacity(32);

    // Flip until we flip a 0, then copy
    for c in flip_bin.chars().rev() {
        match c {
            '0' if !is_added => {
                pls_bin.push('1');
                is_added = true;
            }
            '1' if !is_added => pls_bin.push('0'),
            _ if is_added => pls_bin.push(c),
            _ => unreachable!(),
        }
    }
    pls_bin.chars().rev().collect()
}

#[test]
fn test_convert_binary() {
    // Note abs(i32::MIN) causes overflow due to abs()
    // https://doc.rust-lang.org/std/primitive.i32.html#method.abs
    let vals = [
        i32::MIN + 1,
        -266166237,
        -67841,
        -7,
        -2,
        -1,
        0,
        1,
        2,
        7,
        67841,
        266166237,
        i32::MAX,
    ];

    for t_val in vals {
        let t_exp = format!("{t_val:032b}");
        assert_eq!(convert_binary(&t_val), t_exp);
    }
}

#[test]
fn test_convert_hex() {
    // Note abs(i32::MIN) causes overflow due to abs()
    // https://doc.rust-lang.org/std/primitive.i32.html#method.abs
    let vals = [
        i32::MIN + 1,
        -266166237,
        -67841,
        -7,
        -2,
        -1,
        0,
        1,
        2,
        7,
        67841,
        266166237,
        i32::MAX,
    ];

    for val in vals {
        let t_val = format!("{val:032b}");
        let t_exp = format!("{val:08X}");
        assert_eq!(convert_hex(&t_val), t_exp);
    }
}

#[test]
fn test_pos_fmt_bin() {
    for val in [0, 1, 2, 7, 67841, 266166237, i32::MAX] {
        let t_val = format!("{val:b}");
        let t_exp = format!("{val:032b}");
        assert_eq!(pos_fmt_bin(&t_val), t_exp);
    }
}

#[test]
fn test_pos_int_to_bin() {
    assert!(pos_int_to_bin(&i32::MIN).is_err());
    assert!(pos_int_to_bin(&-1).is_err());

    for t_val in [0, 1, 2, 7, 67841, 266166237, i32::MAX] {
        let t_exp = format!("{t_val:b}");
        assert_eq!(pos_int_to_bin(&t_val), Ok(t_exp));
    }
}

#[test]
fn test_two_comp_flip_bin() {
    for val in [0, 1, 2, 7, 67841, 266166237, i32::MAX] {
        let t_val = format!("{val:032b}");
        let t_exp = format!("{:032b}", !val);
        assert_eq!(two_comp_flip_bin(&t_val), t_exp);
    }
}
