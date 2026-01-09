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
    #[arg(index=1, value_parser=clap::value_parser!(u8))]
    len: Option<u8>,
    /// Set numbers of passwords
    #[arg(index=2, default_value_t = 1, value_parser=clap::value_parser!(u16))]
    num: u16,
    /// Avoid confusing characters like O, 0, I, l, 1
    #[arg(short, long)]
    safe: bool,
}

const CAPITAL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b")-(*&^%$#@!~";
const AMBIGUOUS: &[u8] = b"O0Il1";

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
            generate_secure_password(&mut rng, final_len, args.safe)
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

/// Helper to filter out ambiguous characters if requested
fn get_pool(base: &[u8], avoid: bool) -> Vec<u8> {
    if avoid {
        base.iter()
            .filter(|c| !AMBIGUOUS.contains(c))
            .cloned()
            .collect()
    } else {
        base.to_vec()
    }
}

/// Generates a password
fn generate_secure_password(rng: &mut impl Rng, len: usize, avoid: bool) -> String {
    let cap_pool = get_pool(CAPITAL, avoid);
    let low_pool = get_pool(LOWER, avoid);
    let dig_pool = get_pool(DIGITS, avoid);
    let spec_pool = get_pool(SPECIAL, avoid);

    let mut password: Vec<char> = vec![
        *cap_pool.choose(rng).expect("CAPITAL pool empty") as char,
        *dig_pool.choose(rng).expect("DIGITS pool empty") as char,
        *spec_pool.choose(rng).expect("SPECIAL pool empty") as char,
    ];

    let all_chars: Vec<u8> = [cap_pool, low_pool, dig_pool, spec_pool].concat();
    
    let remaining = len.saturating_sub(password.len());
    password.extend((0..remaining).map(|_| {
        *all_chars.choose(rng).expect("Pool is empty") as char
    }));

    password.shuffle(rng);
    password.into_iter().take(len).collect()
}
