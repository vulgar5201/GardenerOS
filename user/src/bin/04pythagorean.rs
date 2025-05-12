#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let a = 3;
    let b = 4;
    let c = 5;
    
    println!("Checking Pythagorean triplet: {}, {}, {}", a, b, c);
    
    if a*a + b*b == c*c {
        println!("Yes! They form a Pythagorean triplet.");
    } else {
        println!("No! They don't form a Pythagorean triplet.");
    }
    
    0
}