// Ref: https://fasterthanli.me/articles/a-half-hour-to-learn-rust


fn make_string_dangle()->&String {
    let s = String::from("hello");
    &s
}

fn main() {
    // Work!
    let x = 50;
    let _y = x;
    println!("x: {}", x);

    // Not work!
    // let s = String::from("hello");
    // let t = s;
    // println!("t: {}", t);
    // println!("s: {}", s);

    let s = String::from("hello");
    {
        let t = &s;
        println!("t: {}", t);
    }
    println!("s: {}", s);
    println!("s: {}", make_string_dangle());
}
