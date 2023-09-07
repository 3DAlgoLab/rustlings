struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hello_world() {
        dbg!("Hello, world!");
    }

    #[test]
    fn from_into_test() {
        let my_str = "hello";
        let my_string = String::from(my_str);
        dbg!(my_string);

        let int_num = 5;
        let num: Number = int_num.into();
        println!("converted value : {}", num.value);
    }
}
