use rand::{Rng, thread_rng};
use std::char;
use rand::seq::SliceRandom;
use std::str;

fn main() {
    const CAPITAL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &[u8] = b"0123456789";
    const SPECIAL: &[u8] = b")(*&^%$#@!~";
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 5;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let password1: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..DIGITS.len());
            DIGITS[idx] as char
        })
        .collect();

    let l = DIGITS.len() + CAPITAL.len();
    let xs: &[u8] = &[CAPITAL, DIGITS].concat();

    let mut result = rng.gen_range(0..10);
    let password2: String = (0..PASSWORD_LEN-1)
        .map(|_| {
            let idx = rng.gen_range(0..l);
            xs[idx] as char
        })
        .collect();

    let mut pass: Vec<char> = (0..PASSWORD_LEN-1)
        .map(|_| {
            let idx = rng.gen_range(0..l);
            xs[idx] as char
        })
        .collect();

    pass.push(char::from_digit(result as u32, 10).unwrap());

    pass.shuffle(&mut thread_rng());

    let s: String = pass.iter().collect();

    println!("{}", password);
    println!("{}", password1);
    println!("{}", password2);
    println!("{:?}", result);
    println!("{}", s);
}
