
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

// QUESTION 3
// takes two f64 slices that share the same arbitrary length
pub fn distance(first: &[f64], second: &[f64]) -> Result<f64, String> {
    if first.len() == second.len() {
        // do math
        let zipped = first.iter().zip(second.iter());
        let squared_diffs = zipped.map(|p| (p.0 - p.1).powi(2));
        let sum_of_diffs = squared_diffs.fold(0.0, |acc, x| acc + x);
        let dist = sum_of_diffs.sqrt();
        Ok(dist)
    } else {
        // report error
        Err("Slices not of same length".to_string())
    }
}

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