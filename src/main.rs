use clap::{Parser};
use rand::{Rng};
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
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
    #[arg(short, long, default_value_t = 1, value_parser=clap::value_parser!(u8))]
    num: u8,
}

const CAPITAL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b")-(*&^%$#@!~";

fn main() -> ExitCode {
    let args = Cli::parse();
    let mut rng = rand::rng();

    // dynamic defaults based on the 'digit' flag
    let final_len = match args.len {
        Some(l) => l as usize,
        None => if args.digit { 5 } else { 12 },
    };

    for _ in 0..args.num {
        let pass: String = if args.digit {
            generate_pin(&mut rng, final_len)
        } else {
            generate_secure_password(&mut rng, final_len)
        };
        println!("{}", pass);
    }
    ExitCode::SUCCESS
}

/// Generates simple numeric PIN
fn generate_pin(rng: &mut impl Rng, len: usize) -> String {
    (0..len)
        .map(|_| *DIGITS.choose(rng).unwrap() as char)
        .collect()
}

/// Generates a password
fn generate_secure_password(rng: &mut impl Rng, len: usize) -> String {
    let mut password: Vec<char> = vec![
        *CAPITAL.choose(rng).expect("CAPITAL pool is empty") as char,
        *DIGITS.choose(rng).expect("DIGITS pool is empty") as char,
        *SPECIAL.choose(rng).expect("SPECIAL pool is empty") as char,
    ];

    let all_chars: Vec<u8> = [CAPITAL, LOWER, DIGITS, SPECIAL].concat();
    let remaining = len.saturating_sub(password.len());

    password.extend((0..remaining).map(|_| {
        *all_chars.choose(rng).expect("Pool is empty") as char
    }));

    password.shuffle(rng);
    password.into_iter().take(len).collect()
}
