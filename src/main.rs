use clap::{Parser};
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use std::{char, str};
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    digit: bool,
    #[arg(short, long, value_parser=clap::value_parser!(u8))]
    len: Option<u8>,
    #[arg(short, long, value_parser=clap::value_parser!(u8))]
    num: Option<u8>,
}

fn main() -> ExitCode {
    const CAPITAL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const DIGITS: &[u8] = b"0123456789";
    const SPECIAL: &[u8] = b")(*&^%$#@!~";
    let l = CAPITAL.len() + 
        LOWER.len() +
        DIGITS.len() +
        SPECIAL.len();
    let xs: &[u8] = &[CAPITAL, LOWER, DIGITS, SPECIAL].concat();
    let mut rng = rand::thread_rng();
    let ch_capital = CAPITAL.choose(&mut rand::thread_rng());
    let ch_special = SPECIAL.choose(&mut rand::thread_rng());
    let ch_digit = rng.gen_range(0..10);

    let args = Cli::parse();
    let password_len: usize = args.len.unwrap_or(10) as usize;
   
    for _ in 0..args.num.unwrap_or(1) {
        let s: String = if args.digit {
            get_rng(DIGITS, password_len, DIGITS.len())
        } else {
            let mut pass: Vec<char> = get_rng(xs, password_len-3, l);
            pass.push(char::from_digit(ch_digit as u32, 10).unwrap());
            pass.push(*ch_special.unwrap() as char);
            pass.push(*ch_capital.unwrap() as char);

            pass.shuffle(&mut thread_rng());
            pass
        }.iter().collect();
        println!("{}", s);
    }
    ExitCode::SUCCESS
}

fn get_rng(xs: &[u8], l: usize, l1: usize) -> Vec<char> {
    let mut rng = rand::thread_rng();
    (0..l)
        .map(|_| {
            let idx = rng.gen_range(0..l1);
            xs[idx] as char
        })
        .collect()
}
