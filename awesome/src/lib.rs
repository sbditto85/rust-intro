//! the awesome crate is here to help will all your wildest dreams
//! # Example
//! ```
//! assert_eq!(awesome::fizz_buzz(3), "fizz");
//! assert_eq!(awesome::fib(1), 1);
//! ```
use std::borrow::Cow;
use std::fmt::{Display, Error, Formatter};
use std::cmp::PartialEq;

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

/// Fizz Buzz'n function done with enums
/// # Fizz Buzz Enum Example
/// ```
/// # extern crate awesome;
/// # use awesome::fizz_buzz_enum;
/// # fn main () {
///    for n in (0..10).map(fizz_buzz_enum) {
///       println!("{}", n);
///    }
/// # }
/// ```
#[derive(Debug)]
pub enum FizzBuzz {
    FizzBuzz,
    Buzz,
    Fizz,
    Number(u64),
}
trait FizzBuzzer {
    fn fizzbuzz(u64) -> String;
}
impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            FizzBuzz::FizzBuzz  => write!(f, "fizzbuzz"),
            FizzBuzz::Fizz      => write!(f, "fizz"),
            FizzBuzz::Buzz      => write!(f, "buzz"),
            FizzBuzz::Number(n) => write!(f, "{}", n),
        }
    }
}
impl PartialEq for FizzBuzz {
    fn eq(&self, other: &FizzBuzz) -> bool {
        match (self, other) {
            (&FizzBuzz::FizzBuzz, &FizzBuzz::FizzBuzz)                 => true,
            (&FizzBuzz::Fizz, &FizzBuzz::Fizz)                         => true,
            (&FizzBuzz::Buzz, &FizzBuzz::Buzz)                         => true,
            (&FizzBuzz::Number(n1), &FizzBuzz::Number(n2)) if n1 == n2 => true,
            _                                                          => false,
        }
    }
}
pub fn fizz_buzz_enum(n: u64) -> FizzBuzz {
    match (n % 3, n % 5) {
        (0, 0) => FizzBuzz::FizzBuzz,
        (0, _) => FizzBuzz::Fizz,
        (_, 0) => FizzBuzz::Buzz,
        _      => FizzBuzz::Number(n)
    }
}

/// Fib'n your number iterative style
/// # Fib Example
/// ```
/// for n in (1..10).map(awesome::fib) {
///    println!("{}", n);
/// }
/// ```
pub fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        let mut prev = 1;
        let mut now = 1;
        for _ in (1 .. n) { // already done with 2 and range is exclusive for n
            let tmp = now;
            now = prev;
            prev = prev + tmp;
        }
        now
    }
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
            n
        } else {
            f(n,p+n,i-1)
        }
    }
    if n < 2 {
        n
    } else {
        f(1,1, n - 2) //first 2 values are taken care of
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
    fn fizz_buzz_enum_test() {
        let pairs = [(1, FizzBuzz::Number(1)), (3, FizzBuzz::Fizz), (4, FizzBuzz::Number(4)), (5, FizzBuzz::Buzz), (15, FizzBuzz::FizzBuzz)];

        for &(i, ref res) in pairs.iter() {
            assert_eq!(&fizz_buzz_enum(i), res);
        }
        
    }

    #[test]
    fn fib_zero_through_nine(){
        let pairs = [(0, 0), (1, 1), (2, 1), (3, 2), (4, 3),
                     (5, 5), (6, 8), (7, 13), (8, 21), (9, 34)];

        for &(i, res) in pairs.iter() {
            assert_eq!(fib(i), res);
        }
    }
    
    #[test]
    fn fib_rec_zero_through_nine(){
        let pairs = [(0, 0), (1, 1), (2, 1), (3, 2), (4, 3),
                     (5, 5), (6, 8), (7, 13), (8, 21), (9, 34)];

        for &(i, res) in pairs.iter() {
            assert_eq!(fib_rec(i), res);
        }
    }

}
