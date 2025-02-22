use clap::{Parser};
use rand::{Rng};
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use std::{char};
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Generate digit pin
    #[arg(short, long)]
    digit: bool,
    /// Set length of the password
    #[arg(short, long, value_parser=clap::value_parser!(u8))]
    len: Option<u8>,
    /// Set numbers of passwords
    #[arg(short, long, value_parser=clap::value_parser!(u8))]
    num: Option<u8>,
}

fn main() -> ExitCode {
    const DEFAULT_LENGTH: u8 = 10;
    const CAPITAL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const DIGITS: &[u8] = b"0123456789";
    const DIGITS_LEN: usize = 10;
    const SPECIAL: &[u8] = b")-(*&^%$#@!~";
    let l = LOWER.len() * 2 + DIGITS_LEN + SPECIAL.len();

    let xs: &[u8] = &[CAPITAL, LOWER, DIGITS, SPECIAL].concat();
    let mut rng = rand::rng();
    let ch_capital = CAPITAL.choose(&mut rng);
    let ch_special = SPECIAL.choose(&mut rng);
    let ch_digit = rng.random_range(0..DIGITS_LEN);
    
    let args = Cli::parse();
    let password_len: usize = args.len.unwrap_or(DEFAULT_LENGTH) as usize;
   
    for _ in 0..args.num.unwrap_or(1) {
        let s: String = if args.digit {
            get_rng(&mut rng, DIGITS, password_len, DIGITS.len())
        } else {
            let mut pass: Vec<char> = get_rng(&mut rng, xs, password_len-3, l);
            // password will include at least one digit, capital and special char
            pass.push(char::from_digit(ch_digit as u32, 10).unwrap());
            pass.push(*ch_capital.unwrap() as char);
            pass.push(*ch_special.unwrap() as char);

            pass.shuffle(&mut rng);
            pass
        }.iter().collect();
        println!("{}", s);
    }
    ExitCode::SUCCESS
}

fn get_rng(r: &mut ThreadRng, xs: &[u8], l: usize, l1: usize) -> Vec<char> {
    (0..l)
        .map(|_| {
            let idx = r.random_range(0..l1);
            xs[idx] as char
        })
        .collect()
}
