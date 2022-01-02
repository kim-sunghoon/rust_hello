#[macro_use]
extern crate fstrings;

fn main() {
    println!("Hello, world!");
    let name = "Hello";
    let name2 = "World";
    println_f!("{name} {name2}!");

}
