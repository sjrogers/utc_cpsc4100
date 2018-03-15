extern crate num;

fn main() {
    println!("Hello, world!");
}

mod homework {
    use num::{Num};
    use std::ops::Range;


    pub fn yourname() -> String {
        "Sam Rogers".to_string()
    }

    pub fn a_plus_bx<N: Num>(a: N, b: N, c: N) -> N {
        a + b * c
    }

    // pub fn distance<N: Num>(pointA: &Vec<N>, pointB: &Vec<N>) -> N {
    //     let zipped = pointA.iter().zip(pointB);
    //     // zipped;
    //     <N as Num>::from_str_radix("100", 10).ok()
    // }

    // pub fn atoms_only() {}

    pub fn build_list(start: u64, end: u64) -> Range<u64> { start..end }
}

#[cfg(test)]
mod tests {
    use homework::{yourname, a_plus_bx, build_list};

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
}