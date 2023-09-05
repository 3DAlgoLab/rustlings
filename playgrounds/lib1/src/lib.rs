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

pub fn change_string(text: &mut String) {
    text.push_str(" hi hi?");
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

    #[test]
    fn assign_test() {
        let x = 10;
        let y = x;
        println!("x = {}, y = {}", x, y);

        let s = String::from("Hello String");
        // let t = s; // error[E0382]: borrow of moved value: `s`
        // let t = s.clone(); // works but... memory duplication
        let t = &s; // it also works. recommended, borrowing it
        println!("s = {}, t = {}", s, t);
    }

    #[test]
    fn mutable_borrowing() {
        let mut s = String::from("Hello String");
        let t = &s;
        s.push_str(" World");
        // dbg!(t);
        dbg!(s);
    }

    #[test]
    fn mutable_borrowing2() {
        let mut s = String::from("Hello String");
        let t = &mut s;
        t.push_str(" World2");
        // dbg!(t);
        dbg!(s);
    }

    #[test]
    fn mutable_borrowing3() {
        let mut s = String::from("Hello String");
        let t = &mut s;
        change_string(t);
        dbg!(s);
    }

    #[test]
    fn mutable_borrowing4() {
        let mut s = String::from("Hello String");
        change_string(&mut s);
        dbg!(s);
    }

    #[test]
    fn deref_test() {
        let mut name = String::from("John");
        let name_r = &mut name;

        *name_r = String::from("Jane");
        println!("name = {}", name);
    }

    #[test]
    fn deref_test2() {
        let mut x = 30;
        let y = &mut x;
        *y += 1; 
        // dbg!(x, y);
        dbg!(y);
        dbg!(x);
    }

}
