// Write a function that handles the Collatz Sequence
// The Collatz Sequence is defined as follows, for an arbitrary 
// n1 greater than zero:
//    If ni is 1, then the sequence terminates at ni.
//    If ni is even, then ni+1 = ni / 2.
//    If ni is odd, then ni+1 = 3 * ni + 1.
fn collatz(mut n: i32) -> i32 {
    if n < 1 {
        println!{"You must give a non-zero integer!"};
        return 0;
    }
    let mut col_len = 1;
    loop {
        if n == 1 {
            break;
        }
        //if n % 2 == 0 {
        //    n = n / 2;
        //} else {
        //    n = 3 * n + 1;
        //}
        // Here is a more concise oneliner...
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        col_len += 1;
        dbg!(n);
    }
    return col_len;
}

fn main() {
    let arb_int = 3;
    println!("Collatz Length: {}", collatz(arb_int));
}
