const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Hello, world!, {}", OUR_COURSE);
    // stack
    let x: i32;
    x = 2;
    println!("x = {}", x);

    // for
    for i in 0..10 {
        println!("i = {}", i);
    }

    for i in 0..=10 {
        dbg!(i);
    }

    // emoji char
    let emoji = '😀';
    println!("{}", emoji);

    let my_ints = [0; 5];
    // dbg!(my_ints);
    println!("{:?}", my_ints);

    let my_ints_new = my_ints.map(|x| x + 2);
    println!("{:?}", my_ints_new);

}
