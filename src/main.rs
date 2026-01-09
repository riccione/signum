use clap::Parser;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::Rng;
use std::char;
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
    #[arg(index=2, value_parser=clap::value_parser!(u16))]
    num: Option<u16>,
    /// One password per line
    #[arg(short = '1')]
    single_column: bool,
    /// Don't include capital letters in the password
    #[arg(short = 'A', long = "no-capitalize")]
    no_capitalize: bool,
    /// Avoid confusing characters like O, 0, I, l, 1
    #[arg(short = 'B', long = "ambiguous")]
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
        None => {
            if args.digit {
                5
            } else {
                12
            }
        }
    };

    // magic number 156 comes from default grid size
    // 6x26
    let num_to_generate =
        args.num.unwrap_or(if args.single_column { 1 } else { 156 });

    // fixed 80-char width
    const MAX_WIDTH: usize = 80;

    // width of password + 1 space for minimal gutter
    let col_width = final_len + 1;

    // calculate how many fit
    let num_cols = if args.single_column {
        1
    } else {
        (MAX_WIDTH / col_width).max(1)
    };

    for i in 0..num_to_generate {
        let output: String = if args.digit {
            generate_pin(&mut rng, final_len)
        } else {
            generate_secure_password(
                &mut rng,
                final_len,
                args.safe,
                args.no_capitalize,
            )
        };
        if num_cols > 1 {
            // print with fixed padding to keep columns aligned
            print!("{:<width$}", output, width = col_width);

            // newline when the row is full
            if (i + 1) % num_cols as u16 == 0 {
                println!();
            }
        } else {
            println!("{}", output);
        }
    }

    // final newline if the loop ended mid-row
    if num_cols > 1 && num_to_generate % num_cols as u16 != 0 {
        println!();
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
fn generate_secure_password(
    rng: &mut impl Rng,
    len: usize,
    avoid: bool,
    no_caps: bool,
) -> String {
    let cap_pool = if no_caps {
        vec![]
    } else {
        get_pool(CAPITAL, avoid)
    };
    let low_pool = get_pool(LOWER, avoid);
    let dig_pool = get_pool(DIGITS, avoid);
    let spec_pool = get_pool(SPECIAL, avoid);

    let mut password: Vec<char> = Vec::new();
    password.push(*low_pool.choose(rng).expect("LOWER empty") as char);
    password.push(*dig_pool.choose(rng).expect("DIGITS pool empty") as char);
    password.push(*spec_pool.choose(rng).expect("SPECIAL pool empty") as char);

    if !no_caps {
        password.push(*cap_pool.choose(rng).expect("CAPITAL empty") as char);
    }

    let all_chars: Vec<u8> =
        [&cap_pool[..], &low_pool[..], &dig_pool[..], &spec_pool[..]].concat();

    let remaining = len.saturating_sub(password.len());
    password.extend(
        (0..remaining)
            .map(|_| *all_chars.choose(rng).expect("Pool is empty") as char),
    );

    password.shuffle(rng);
    password.into_iter().take(len).collect()
}
