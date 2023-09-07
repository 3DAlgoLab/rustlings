mod m10_proc_macro;
mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetime;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;

my_proc_macro::create_struct!(Point);

#[allow(dead_code, unused_variables)]
// #[tokio::main]
fn main() {
    let point = Point { x: 1, y: 2 };
    print!("Point: ({}, {})", point.x, point.y);
}
