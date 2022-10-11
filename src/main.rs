use std::str::FromStr;
use crate::break_palindrome::break_palindrome;

pub mod break_palindrome;

fn main() {
    let res = break_palindrome(String::from_str("aba").unwrap());
    println!("{0}", res);
}
