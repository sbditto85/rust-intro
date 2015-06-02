//! the awesome crate is here to help will all your wildest dreams
//! # Example
//! ```
//! assert_eq!(awesome::fizz_buzz(3), "fizz");
//! assert_eq!(awesome::fib(1), 0);
//! ```
use std::borrow::Cow;

/// Fizz Buzz'n function
/// # Fizz Buzz Example
/// ```
/// # extern crate awesome;
/// # use awesome::fizz_buzz;
/// # fn main () {
///    for n in (0..10).map(fizz_buzz) {
///       println!("{}", n);
///    }
/// # }
/// ```
pub fn fizz_buzz(n: u64) -> Cow<'static, str> {
    match (n % 3, n % 5) {
        (0, 0) => "fizzbuzz".into(),
        (0, _) => "fizz".into(),
        (_, 0) => "buzz".into(),
        _      => n.to_string().into()
    }
}

/// Fib'n your number iterative style 1 is the first fib number
/// # Fib Example
/// ```
/// for n in (1..10).map(awesome::fib) {
///    println!("{}", n);
/// }
/// ```
pub fn fib(n: u64) -> u64 {
    let mut prev = 0;
    let mut now = 1;
    for _ in (0 .. n) {
        let tmp = now;
        now = prev;
        prev = prev + tmp;
    }
    now
}

/// Fib'n your number in the new recursive awesome way
/// # Fib Recursive Example
/// ```
/// for n in (1..10).map(awesome::fib_rec) {
///    println!("{}", n);
/// }
/// ```
pub fn fib_rec(n: u64) -> u64 {
    fn f(p: u64, n: u64, i: u64) -> u64 {
        if i == 0 {
            p + n
        } else {
            f(n,p+n,i-1)
        }
    }
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        f(0,1, n - 2) //first 2 values are taken care of
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fizz_buzz_one_through_fifteen() {
        let pairs = [(1, "1"), (2, "2"), (3, "fizz"), (4, "4"), (5, "buzz"),
                     (6, "fizz"), (7, "7"), (8, "8"), (9, "fizz"), (10, "buzz"),
                     (11, "11"), (12, "fizz"), (13, "13"), (14, "14"), (15, "fizzbuzz")];

        for &(i, res) in pairs.iter() {
            assert_eq!(fizz_buzz(i), res);
        }
        
    }

    #[test]
    fn fib_one_through_ten(){
        let pairs = [(1, 0), (2, 1), (3, 1), (4, 2), (5, 3),
                     (6, 5), (7, 8), (8, 13), (9, 21), (10, 34)];

        for &(i, res) in pairs.iter() {
            assert_eq!(fib(i), res);
        }
        
    }

}
