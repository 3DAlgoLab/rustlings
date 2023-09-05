pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.as_ref().unwrap();
    s.clone()
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
    fn it_gets_or_default() {
        let result = get_or_default(&None);
        assert_eq!(result, "");
        let out = get_or_default(&Some("hello".to_string()));
        dbg!(out);
    }

    #[test]
    fn str_test() {
        let s = String::from("Hello String");
        println!("{}", &s[0..3]);
    }
}
