#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    macro_rules! mad_skills {
        // ($x:expr) => {
        //     format!("You send an expression: {}", $x)
        // };
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string(),
            }
        };
    }

    macro_rules! my_vec {
        ($($x: expr), +) => {
            {
                let mut temp_vec = Vec::new();
                $(temp_vec.push($x);)+
                temp_vec
            }
        };
    }

    #[test]
    fn test_decl_macro() {
        println!("Hello 0");
        dbg!("hello");
        let x = vec![1, 2, 3];
        let formatted = format!("hi hi there {x:?}");
        dbg!(formatted);
    }

    #[test]
    fn test_custom() {
        // let some = mad_skills!(1 + 2);
        // dbg!(some);
        let some_var = mad_skills!(i32);
        dbg!(some_var);
        let some_var = mad_skills!(u8);
        dbg!(some_var);
    }

    #[test]
    fn test_custom2() {
        let mut y = my_vec!(1, 2, 3);
        dbg!(y);
    }
}
