use std::io::Write;
use std::str::FromStr;

/*
   fn is a function declaration
   gcd is the function name
   mut means this parameter or variable is mutatable. Otherwise it's a constant by default.
   n and m are variable names
   u64 are the types
   -> u64 means the return type of the function
*/

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // This checks these values and if not, it blows and 'panics'
                               // Convention is to leave out the (). But it will still work.
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    // equivalent to `return n;` ONLY if you LEAVE OUT THE ;
    // This is the general convention in Rust.
    // return n;
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 13), 1);

    assert_eq!(gcd(5, 15), 5);
}

fn main() {
    // A flexible array type thing. The type declaration is optional, the compiler can infer it
    let mut numbers: Vec<u64> = Vec::new();
    // How to read command line arguments.
    // args() returns an iterator, so that's how it knows how to for-loop
    // we skip the first argument, because it's always going to be the name of the program running, and
    // not actually a command line argument
    // &arg borrows a reference to arg. The ownership remains with arg, we just want to be able to access it
    // *arg would get the value by dereferencing arg and reading it
    // Chapter 4 covers it in more details
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).unwrap())
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // &numbers borrows a reference to numbers.
    // The ownership remains with arg, we just want to be able to access it
    // *arg would get the value by dereferencing arg and reading it
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    // This is how you sub in values into a string
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
