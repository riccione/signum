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
        if args.digit {
            let password2: String = (0..password_len)
                .map(|_| {
                    let idx = rng.gen_range(0..password_len);
                    DIGITS[idx] as char
                })
                .collect();
            println!("{password2}");
        } else {
            let mut pass: Vec<char> = (0..password_len-3)
                .map(|_| {
                    let idx = rng.gen_range(0..l);
                    xs[idx] as char
                })
                .collect();

            pass.push(char::from_digit(ch_digit as u32, 10).unwrap());
            pass.push(*ch_special.unwrap() as char);
            pass.push(*ch_capital.unwrap() as char);

            pass.shuffle(&mut thread_rng());

            let s: String = pass.iter().collect();

            println!("{}", s);
        }
    }
    ExitCode::SUCCESS
}
