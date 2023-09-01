use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

fn _test1() {
    let greeting_file_result = File::open("greeting.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("{:?}", greeting_file);
}

fn _test2() {
    let greeting_file = File::open("hello.txt").unwrap();
    println!("{:?}", greeting_file);
}

fn _test3() {
    let greeting_file = File::open("hello.txt").expect("file is not existed");
    println!("{:?}", greeting_file);
}

fn _test4() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello1.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    // let greeting_file_result = File::open("greeting.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // println!("{:?}", greeting_file);
    let r = _test4()?;
    println!("{:?}", r);
    Ok(())
}
