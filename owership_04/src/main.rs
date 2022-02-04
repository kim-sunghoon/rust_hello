#[macro_use]
extern crate fstrings;

fn main() {
    let mut s1 = String::from("hello");
    
    let len = calculate_length(&mut s1);
    let s3 = s1.clone();
    
    println!("The length of '{}' is {}.", s1, len);
    s1.push_str(", world!"); 
    println_f!("{s1}, {s3}"); 
    
    let hello = &s1[0..5];
    let world = &s1[6..];

    println_f!("{hello} {world}");

}

fn calculate_length(s: &mut String) -> usize {
    let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.

    length
}