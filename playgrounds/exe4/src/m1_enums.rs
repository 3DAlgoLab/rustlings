#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver,
}

impl Default for CarColor {
    fn default() -> Self {
        Self::Red
    }
}

// #[derive(Debug)]
// enum GivenResult<T, E> {
//     Ok(T),
//     Err(E),
// }

fn check_under_five(num: u8) -> Result<u8, String> {
    if num < 5 {
        Ok(num)
    } else {
        Err("Not under 5".to_string())
    }
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero2(num_check: f32) -> Option<f32> {
    let remainder = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let c_color = {
            let color1 = CarColor::default();
            color1
        };
        dbg!(c_color);
    }

    #[test]
    fn test_under_5() {
        let is_ok_res = check_under_five(6);
        dbg!(is_ok_res);

        let is_ok_res = check_under_five(4);
        dbg!(is_ok_res);
    }

    #[test]
    fn test_remainder_zero() {
        let r = remainder_zero(29.2222);
        dbg!(r);

        let r = remainder_zero(20.000);
        dbg!(r);
    }

    #[test]
    fn test_remainder_zero2() {
        let r = remainder_zero2(29.2222);
        dbg!(r);

        let r = remainder_zero2(20.000);
        dbg!(r);
    }
}
