
//! This module contains misc. functions for a course assignnemnt.
//! author: Sam Rogers
//! Interested parties may use this code under terms of the WTFPL.
extern crate num;

use std::ops::Range;

// QUESTION 1
pub fn yourname() -> String {
    "Sam Rogers".to_string()
}

// QUESTION 2
pub fn a_plus_bx<N: num::Num>(a: N, b: N, c: N) -> N {
    a + b * c
}

// pub fn distance<N: Num>(pointA: &Vec<N>, pointB: &Vec<N>) -> N {
//     let zipped = pointA.iter().zip(pointB);
//     // zipped;
//     <N as Num>::from_str_radix("100", 10).ok()
// }

// QUESTION 4
/* In the context of Rust, we will define atoms as values that are
 * immutable AND cannot be broken down. This rules out collections
 * as well as strings. We can achieve an "atoms only" filter fn
 * by means of pattern-matching with traits that disqualify.
 */
// pub fn atoms_only() {}

// QUESTION 5
pub fn build_list(start: u64, end: u64) -> Range<u64> { start..end }

// QUESTION 6
// MAPPING CLOSURES IS HARDER IN RUST!
pub fn diff(origin: i64, lst: Vec<i64>) -> Vec<i64> {
    let f = |x: &i64| { (x - origin).abs() };
    lst.iter().map(f).rev().collect()
}
// pub fn diff<N>(origin: N, lst: Vec<N>) -> Vec<N>
//     where N: num::Signed {
//     // let f = |n: N| -> N { n - origin };
//     let mut result = lst.iter().map(|&n| -> N { n });//.rev().collect();
//     result.rev().collect()
// }

// QUESTION 7

// QUESTION 8
pub fn convert(opname: &str, arg: &str) -> String {
    match opname {
        "upper" => arg.to_string().to_uppercase(),
        "lower" => arg.to_string().to_lowercase(),
        "reverse" => arg.chars().rev().collect::<String>(),
        "size" => arg.to_string().len().to_string(),
        _ => opname.to_string()
    }
}

// QUESTION 9
pub fn strip_spaces(original: &str) -> String {
    original.replace(" ", "").to_string()
}

// QUESTION 10