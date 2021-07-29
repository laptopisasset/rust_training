
use std::mem;

fn main() {
    let a: u8 = 123; // u = unsigned, 8 bits, 0 - 255
    println!("a = {}", a); // immutable

    // u = unsigned, 0 to 2^N-1
    // i = signed, -2^N-1
    let mut b: i8 = 0; // -128 -- 127
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let c = 123456789; // i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
}
