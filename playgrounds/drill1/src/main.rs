// use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // let n = 10;
    // let mut s = String::from("Hello, world!");

    // for i in 0..n {
    //     // Append string n times
    //     // s.push_str(" -- ");
    //     // s.push_str(&i.to_string());
    //     // format?
    //     s = format!("{} -- {}", s, i);
    // }

    // println!("{}", s);

    // for i in 0..n {
    //     println!("Hello, world! -- {}", i);
    // }
    // let mut map = HashMap::new();
    // map.insert("one", 1);
    // map.insert("two", 2);

    // // let value = map.get("one");
    // // let incremented_value = value + 1;
    // let incremented_value = match map.get("one") {
    //     Some(value) => value + 1,
    //     None => 0,
    // };
    // println!("{}", incremented_value);
    // Options
    // let my_result = Ok::<&str, ()>("okay good!");
    // let my_err = Err::<(), f32>(0.0);
    // println!("{:?} {:?}", my_result, my_err);
    // println!("End!");

    let path = Path::new("data.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {:?}", err),
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    println!("{}", s);
}
