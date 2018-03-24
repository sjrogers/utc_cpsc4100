extern crate librogers;

#[cfg(test)]
mod tests {
    use librogers::homework::{yourname, a_plus_bx, build_list, diff};

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