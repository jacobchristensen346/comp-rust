// Demonstrating how Rust can infer variable types

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn takes_u16(x: u16) {
    println!("u16: {x}");
}

fn main() {
    let x = interproduct(120, 100, 248);
    println!("result: {x}");
    let y = 10000;
    takes_u16(y);
}
