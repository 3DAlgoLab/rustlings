fn main() {
    let name = "Rust";
    println!("Hello, {}!", name);

    let dynamic_string = String::from("Rust Language");
    dbg!(&dynamic_string);

    // let dynamic_string = name.to_string();
    // dbg!(dynamic_string);

    let str_slice = &dynamic_string[0..3];
    dbg!(str_slice);

    // vec
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'K');
    chars.insert(1, 'O');
    chars.insert(2, 'R');
    chars.push('E');
    chars.push('A');
    dbg!(&chars);

    //
    let removed_char = chars.pop().unwrap();
    dbg!(removed_char);

    let chars_again = vec!['K', 'O', 'R', 'E', 'A'];
    dbg!(&chars_again);

    let collected = chars_again.iter().collect::<String>();
    dbg!(collected);

    // closures
    let num = 5;
    let add_num = |x: i32| x + num;
    dbg!(add_num(num));

    // Literals
    println!("Big Number is {}", 98_222_000);
    println!("Hex Number is {}", 0xff);
    println!("Octal Number is {}", 0o77);
    println!("Binary Number is {}", 0b1111_0000);
    println!("Byte Number is {}", b'A');

    // RAW JSON
    let text = r#"{
        "name": "Rust",
        "rank": 1,
        "version": 1.50
    }"#;
    dbg!(text);
    // Binary 
    let a:u8 = 0b_1010_1010;
    let b:u8 = 0b_0101_1010;
    println!("a & b = {:08b}", a & b);
    println!("a OR b = {:08b}", a | b);
    println!("a XOR b = {:08b}", a ^ b);
    println!("NOT a = {:08b}", !a);
    
      
}
