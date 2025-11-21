// Exercise: Write a function to handle the Fibonacci sequence.
// Have it return the nth number in the sequence starting with [0, 1].

fn fib_seq(n: i32) -> i32 {
    let mut idx = 0;
    let mut fnum = 0;
    let mut snum = 1;
    let mut nth_num = 0;
    while idx <= n { // Iterates from 0 to n (inclusive)
        if idx == 0 { // Account for first two numbers
            nth_num = 0;
            idx += 1;
        } else if idx == 1 {
            nth_num = 1;
            idx += 1;
        } else { // Then carry on with the pattern
            nth_num = fnum + snum;
            fnum = snum;
            snum = nth_num;
            idx += 1;
        }
    }
    return nth_num;
}

fn main() {
    println!("result: {}", fib_seq(7));
}
