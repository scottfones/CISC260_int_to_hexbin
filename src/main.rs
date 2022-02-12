//! CISC 260 - HW 1
//! Integer to Hexadecimal and Binary

use std::cmp::Ordering::*;
use std::io;

fn main() {
    let input = get_input();
    let fmt_bin = convert_binary(&input);

    println!("Integer: {input}");
    println!("Binary: {fmt_bin}");
}

/// Returns a formatted binary String, given an i32.
///
/// Note: `int` cannot be i32::MIN due to abs() overflow,
/// See https://doc.rust-lang.org/std/primitive.i32.html#method.abs
fn convert_binary(int: &i32) -> String {
    let raw_bin = pos_int_to_bin(&int.abs()).unwrap();
    let pfmt_bin = pos_fmt_bin(&raw_bin);
    two_comp_fmt_bin(&pfmt_bin, int.is_negative())
}

/// Get user input and return as 32-bit integer.
fn get_input() -> i32 {
    let mut input = String::new();

    println!("Enter your 32-bit integer: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().parse::<i64>().unwrap() as i32
}

/// Formats a truncated binary string to 31-bit by padding zeros.
fn pos_fmt_bin(raw_bin: &String) -> String {
    match raw_bin.len().cmp(&31) {
        Less => pos_fmt_bin(&format!("0{raw_bin}")),
        Equal => raw_bin.to_string(),
        _ => unreachable!(),
    }
}

/// Converts a positive, 32-bit integer to a binary string representation.
/// If the value is negative, return None.
fn pos_int_to_bin(int: &i32) -> Option<String> {
    if int.lt(&0) {
        return None;
    }
    if int.le(&1) {
        return Some(int.to_string());
    }

    match int % 2 {
        0 => Some(pos_int_to_bin(&(int / 2)).unwrap() + "0"),
        1 => Some(pos_int_to_bin(&(int / 2)).unwrap() + "1"),
        _ => unreachable!(),
    }
}

fn two_comp_fmt_bin(raw_bin: &String, is_neg: bool) -> String {
    match is_neg {
        false => format!("0{raw_bin}"),
        true => {
            let comp_bin: String = raw_bin
                .chars()
                .map(|c| match c {
                    '0' => '1',
                    '1' => '0',
                    _ => unreachable!(),
                })
                .collect();
            format!("1{comp_bin}")
        }
    }
}

#[test]
fn test_convert_binary() {
    // Note abs(i32::MIN) causes overflow due to abs()
    // https://doc.rust-lang.org/std/primitive.i32.html#method.abs
    let t_val = i32::MIN + 1;
    let t_exp = "10000000000000000000000000000000".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = -2022;
    let t_exp = "11111111111111111111100000011001".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = -7;
    let t_exp = "11111111111111111111111111111000".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = -2;
    let t_exp = "11111111111111111111111111111101".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = -1;
    let t_exp = "11111111111111111111111111111110".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = 0;
    let t_exp = "00000000000000000000000000000000".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = 1;
    let t_exp = "00000000000000000000000000000001".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = 2;
    let t_exp = "00000000000000000000000000000010".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = 7;
    let t_exp = "00000000000000000000000000000111".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

    let t_val = 2022;
    let t_exp = "00000000000000000000011111100110".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);


    let t_val = i32::MAX;
    let t_exp = "01111111111111111111111111111111".to_string();
    assert_eq!(t_exp.len(), 32);
    assert_eq!(convert_binary(&t_val), t_exp);

}

#[test]
fn test_pos_fmt_bin() {
    // 0
    assert_eq!(
        pos_fmt_bin(&"0".to_string()),
        "0000000000000000000000000000000".to_string()
    );

    // 1
    assert_eq!(
        pos_fmt_bin(&"1".to_string()),
        "0000000000000000000000000000001".to_string()
    );

    // 2
    assert_eq!(
        pos_fmt_bin(&"10".to_string()),
        "0000000000000000000000000000010".to_string()
    );

    // 7
    assert_eq!(
        pos_fmt_bin(&"111".to_string()),
        "0000000000000000000000000000111".to_string()
    );

    // 2022
    assert_eq!(
        pos_fmt_bin(&"11111100110".to_string()),
        "0000000000000000000011111100110".to_string()
    );

    // i32::MAX
    assert_eq!(
        pos_fmt_bin(&"1111111111111111111111111111111".to_string()),
        "1111111111111111111111111111111".to_string()
    );
}

#[test]
fn test_pos_int_to_bin() {
    assert_eq!(pos_int_to_bin(&i32::MIN), None);
    assert_eq!(pos_int_to_bin(&-1), None);

    assert_eq!(pos_int_to_bin(&0), Some("0".to_string()));
    assert_eq!(pos_int_to_bin(&1), Some("1".to_string()));
    assert_eq!(pos_int_to_bin(&2), Some("10".to_string()));
    assert_eq!(pos_int_to_bin(&7), Some("111".to_string()));
    assert_eq!(pos_int_to_bin(&2022), Some("11111100110".to_string()));
    assert_eq!(
        pos_int_to_bin(&i32::MAX),
        Some("1111111111111111111111111111111".to_string())
    );
}

#[test]
fn test_two_comp_fmt_bin() {
    // Negative i32::MAX
    let t_val = "1111111111111111111111111111111".to_string();
    let t_exp = "10000000000000000000000000000000".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, true), t_exp);

    // Negative 2022
    let t_val = "0000000000000000000011111100110".to_string();
    let t_exp = "11111111111111111111100000011001".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, true), t_exp);

    // Negative 7
    let t_val = "0000000000000000000000000000111".to_string();
    let t_exp = "11111111111111111111111111111000".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, true), t_exp);

    // Negative 2
    let t_val = "0000000000000000000000000000010".to_string();
    let t_exp = "11111111111111111111111111111101".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, true), t_exp);

    // Negative 1
    let t_val = "0000000000000000000000000000001".to_string();
    let t_exp = "11111111111111111111111111111110".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, true), t_exp);

    // 0
    let t_val = "0000000000000000000000000000000".to_string();
    let t_exp = "00000000000000000000000000000000".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, false), t_exp);

    // 1
    let t_val = "0000000000000000000000000000001".to_string();
    let t_exp = "00000000000000000000000000000001".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, false), t_exp);

    // 2
    let t_val = "0000000000000000000000000000010".to_string();
    let t_exp = "00000000000000000000000000000010".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, false), t_exp);

    // 7
    let t_val = "0000000000000000000000000000111".to_string();
    let t_exp = "00000000000000000000000000000111".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, false), t_exp);

    // 2022
    let t_val = "0000000000000000000011111100110".to_string();
    let t_exp = "00000000000000000000011111100110".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, false), t_exp);

    // i32::MAX
    let t_val = "1111111111111111111111111111111".to_string();
    let t_exp = "01111111111111111111111111111111".to_string();
    assert_eq!(t_val.len(), 31);
    assert_eq!(t_exp.len(), 32);
    assert_eq!(two_comp_fmt_bin(&t_val, false), t_exp);
}
