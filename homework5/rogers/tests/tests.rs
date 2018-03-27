extern crate librogers;

#[cfg(test)]
mod tests {
    use librogers::homework::{yourname, a_plus_bx, build_list, diff, distance, enumerate, convert, strip_spaces};

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
        let expected = 10.0;
        let test_a = [8f64, 0f64, 0f64];
        let test_b = [8f64, 10f64, 0f64];
        assert_eq!(distance(&test_a, &test_b), Ok(expected))
    }

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

    #[test]
    fn question7() {
        let invals = "abcdefghijklmnopqrstuvwxyz".to_string();
        let expected: Vec<_> = invals.chars().enumerate().collect();
        // assert_eq!(enumerate(invals.chars()), expected)
        for (actual, intended) in enumerate(invals.chars()).zip(expected) {
            assert_eq!(actual, intended);
        }
    }

    #[test]
    fn question8() {
        let s = yourname();
        assert_eq!(convert("upper", &s), s.to_string().to_uppercase());
        assert_eq!(convert("lower", &s), s.to_string().to_lowercase());
        assert_eq!(convert("reverse", &s), s.chars().rev().collect::<String>());
        assert_eq!(convert("size", &s), s.to_string().len().to_string());
        assert_eq!(convert("invalid_op", &s), "invalid_op".to_string());
    }

    #[test]
    fn question9() {
        let spaced = "this is a test";
        let result = strip_spaces(spaced);
        assert_eq!(result, "thisisatest");
    }
}