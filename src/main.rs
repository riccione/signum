use clap::{Parser};
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use std::{char, str};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    digit: bool,
    #[arg(short, long, value_parser=(clap::value_parser!(u8)))]
    len: Option<u8>,
}

fn main() {
    let args = Cli::parse();
    println!("digit: {}", args.digit);
    println!("len: {}", args.len.unwrap_or(10));

    const CAPITAL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
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

    let l = CAPITAL.len() + 
        LOWER.len() +
        DIGITS.len() +
        SPECIAL.len();
    let xs: &[u8] = &[CAPITAL, LOWER, DIGITS, SPECIAL].concat();

    let ch_special = SPECIAL.choose(&mut rand::thread_rng());
    let ch_digit = rng.gen_range(0..10);
    
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

    println!("{:?}", *ch_special.unwrap() as char);
}
