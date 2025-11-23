// Exercise: Write a function to handle the Fibonacci sequence.
// Have it return the nth number in the sequence starting with [0, 1].
// This time use recursion.

fn fib_seq(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 0 {
        return 0;
    } else {
        return fib_seq(n - 1) + fib_seq(n - 2);
    }
}

fn main() {
    println!("result: {}", fib_seq(2));
}
