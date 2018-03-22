extern crate num;

fn main() {
    println!("Hello, world!");
}

mod homework {
    use num::{Num};
    use std::ops::Range;

    // QUESTION 1
    pub fn yourname() -> String {
        "Sam Rogers".to_string()
    }

    // QUESTION 2
    pub fn a_plus_bx<N: Num>(a: N, b: N, c: N) -> N {
        a + b * c
    }

    // pub fn distance<N: Num>(pointA: &Vec<N>, pointB: &Vec<N>) -> N {
    //     let zipped = pointA.iter().zip(pointB);
    //     // zipped;
    //     <N as Num>::from_str_radix("100", 10).ok()
    // }

    // QUESTION 4
    // In the context of Rust, we define an atom as anything immutable or of a primitive type.
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

    // QUESTION 9

    // QUESTION 10
}

#[cfg(test)]
mod tests {
    use homework::{yourname, a_plus_bx, build_list, diff};

    #[test]
    fn question1() {
        assert!(yourname() == "Sam Rogers".to_string())
    }

    #[test]
    fn question2() {
        assert!(a_plus_bx(1, 2, 3) == 7)
    }

    // #[test]
    // fn question3() {
    //     assert!(distance())
    // }

    #[test]
    fn question5() {
        let s = 10;
        let e = 100;
        let expected = s..e;
        let result = build_list(s, e);
        assert!(result == expected)
    }

    #[test]
    fn question6() {
        let o = 10;
        let lst = vec![8, 9, 10, 11, 12];
        let expected = vec![2, 1, 0, 1, 2];
        let result = diff(o, lst);
        assert_eq!(result, expected)
    }
}