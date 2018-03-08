extern crate num;
use num::Num;

fn main() {
    println!("Hello, world!");
}

fn yourname() -> String {
    "Sam Rogers".to_string()
}

fn a_plus_bx<N: Num>(a: N, b: N, c: N) -> N {
    a + b * c
}
#[cfg(test)]
mod tests {
    use {yourname, a_plus_bx};
    #[test]
    fn question1() {
        assert!(yourname() == "Sam Rogers".to_string())
    }

    #[test]
    fn question2() {
        assert!(a_plus_bx(1, 2, 3) == 7)
    }
}