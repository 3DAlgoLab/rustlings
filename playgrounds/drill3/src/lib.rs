pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn _make_string_dangle() -> String {
    String::from("hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn dangle() {
        let s = _make_string_dangle();
        assert_eq!(s, "hello");
    }

    #[test]
    fn mutable_test() {
        let mut s = String::from("hello");
        let t = &s;
        // s.push_str(", world!");
        assert_eq!(t, "hello");
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mutable_test2() {
        let mut s = String::from("hello");
        let t = &mut s;
        t.push_str(", world");
        // s.push_str("~");
        // assert_eq!(t, "hello, world");
        dbg!(t);
        assert_eq!(s, "hello, world");
    }
}
