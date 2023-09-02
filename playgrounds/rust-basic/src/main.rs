// Ref: https://fasterthanli.me/articles/a-half-hour-to-learn-rust

fn main() {
    // let wrapped = Some(2);
    // let empty: Option<i32> = None;
    // println!("wrapped: {:?}", wrapped);
    // println!("empty: {:?}", empty);

    // let r1 = Ok(64);
    // let r2 = Err("oh no!");

    // let x = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // x.iter().for_each(|i| println!("{}", i));
    // let z = x.iter().map(|x| x+ 3).fold(0, |x, y| x+y);
    // dbg!(z);

    let x = "out";
    {
        let x = "in";
        println!("inner x: {}", x);
    }
    println!("outer x: {}", x);
    let x = 42;
    println!("outer x: {}", x);
    let x = { 100 };
    println!("outer x: {}", x);
}
