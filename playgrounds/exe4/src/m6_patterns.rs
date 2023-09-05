#[allow(dead_code, unused_variables)]
#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(u8, u8, u8),
    Move { x: i32, y: i32 },
    Write(String),
}

fn _process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I'm quit")
        }
        Message::ChangeColor(r, g, b) => {
            println!("Red {r}, Green {g}, Blue {b}")
        }
        Message::Move { x, y: new_name } => {
            println!("X is {x}, Y as new_name is {new_name}")
        }
        Message::Write(text) => {
            println!("{}", text);
        }
    }
}

#[cfg(test)]
#[allow(unused)]
mod test {

    use super::*;

    #[test]
    fn test_match_literal() {
        let number = 5;
        let result = match number {
            1 => "First",
            2 | 3 | 5 | 7 => "Other numbers 2 ~ 7",
            _ => "Something else... ",
        };

        dbg!(result);
    }

    #[test]
    fn test_match_option() {
        let some_num = Some(10);
        // let some_num = None;
        // let prob_none: Option<i32> = None;

        // let r = match some_num {
        //     Some(i) => i,
        //     None => {
        //         panic!("Critical Problem");
        //     }
        // };
        // dbg!(r);

        // Similar version with 'if else' statement
        if let Some(i) = some_num {
            dbg!(i);
        } else {
            panic!("Critical Problem!")
        }
    }

    #[test]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        let _some_err: Result<i32, &str> = Err("There was a problem");
        let r = match some_res {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        dbg!(r);
    }

    #[test]
    fn tests_match_result2() {
        let some_res: Result<i32, &str> = Ok(50);
        let _some_err: Result<i32, &str> = Err("There was a problem");

        let result = if let Ok(r) = some_res {
            r
        } else {
            panic!("There was a problem!");
        };

        dbg!(result);
    }

    #[test]
    fn test_match_msg() {
        let my_enum = Message::Quit;
        _process_message(my_enum);

        let my_msg = Message::Move { x: 10, y: 20 };
        _process_message(my_msg);

        let my_msg2 = Message::ChangeColor(10, 20, 30);
        _process_message(my_msg2);
    }

    #[test]
    fn test_match_guard() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("They match"),
            (x, y) if x + y == 0 => println!("They neutralized"),
            (_, y) if y == 2 => println!("Y indeed +2"),
            _ => println!("Everything else"),
        }
    }

    #[test]
    fn test_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let location = Location { x: 11, y: 0 };
        match location {
            Location { x, y: 0 } if x < 10 => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("X and Y are not on the axis"),
        };
    }
}
