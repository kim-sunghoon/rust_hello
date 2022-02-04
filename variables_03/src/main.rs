#[macro_use]
extern crate fstrings;

fn main() {
    let mut x = 5;
    println_f!("The value of x is: {x}");
    x = 6;
    println_f!("The value of x is: {x}");

    let t = true;
    let f: bool = false; // with explicit type annotation
    println_f!("The value of t={t}, f={f}");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println_f!("{c}, {z}, {heart_eyed_cat}");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println_f!("the value of a with index: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("the value with for: {}", element);
    }


    for number in (1..4).rev() {
        println_f!("{number} with rev!");
    }
    println!("LIFTOFF!!!");
}
