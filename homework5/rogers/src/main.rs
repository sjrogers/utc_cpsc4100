extern crate num;

fn main() {
    println!("Hello, world!");
}

mod homework {
    use num::Num;
    pub fn yourname() -> String {
        "Sam Rogers".to_string()
    }

    pub fn a_plus_bx<N: Num>(a: N, b: N, c: N) -> N {
        a + b * c
}

    pub fn distance() -> bool { true }
}

#[cfg(test)]
mod tests {
    use homework::{yourname, a_plus_bx, distance};

    #[test]
    fn question1() {
        assert!(yourname() == "Sam Rogers".to_string())
    }

    #[test]
    fn question2() {
        assert!(a_plus_bx(1, 2, 3) == 7)
    }

    #[test]
    fn question3() {
        assert!(distance())
    }
}