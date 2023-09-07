use std::collections::HashMap;

fn main() {
    let mut profile = HashMap::new();
    profile.insert("name", CharacterValue::Name("John".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert(
        "items",
        CharacterValue::Items(vec![
            "laptop".to_string(),
            "book".to_string(),
            "coat".to_string(),
        ]),
    );

    println!("{:?}", profile);
}

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}

fn _print(value: &String) {
    println!("{}", value);
}

fn _print2(value: &String, value_two: &String) {
    println!("{}", value);
    println!("{}", value_two);
}

fn _print3(value: &mut i8) {
    *value += 1;
    println!("In function the value is {}", value);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_memory_handling() {
        let one = "one".to_string();
        let two = one.to_owned() + " two";

        println!("{}", one);
        println!("{}", two);
    }

    #[test]
    fn test_borrow() {
        let one = "one".to_string();
        _print(&one);
        println!("{}", one);
    }

    #[test]
    fn test_borrow2() {
        let one = "one".to_string();
        _print2(&one, &one);
        println!("{}", one);
    }

    #[test]
    fn test_borrow3() {
        let mut one: i8 = 5;
        _print3(&mut one);
        println!("In host the value is {}", one);
    }

    #[test]
    fn test_closure() {
        let test_closure = |string_input|{
            println!("{}", string_input);
        };
        
        test_closure(32);
        test_closure("hi closure?");
    }
}
