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
    /// Remove characters from the set of characters to generate password
    #[arg(short = 'r', long = "remove-chars")]
    remove_chars: Option<String>,
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
                &args.remove_chars,
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
fn get_pool(
    base: &[u8],
    avoid: bool,
    custom_exclude: &Option<String>,
) -> Vec<u8> {
    base.iter()
        .filter(|&&c| {
            if avoid && AMBIGUOUS.contains(&c) {
                return false;
            }
            if let Some(exclude) = custom_exclude {
                if exclude.as_bytes().contains(&c) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

/// Generates a password
fn generate_secure_password(
    rng: &mut impl Rng,
    len: usize,
    avoid: bool,
    no_caps: bool,
    custom_exclude: &Option<String>,
) -> String {
    let cap_pool = if no_caps {
        vec![]
    } else {
        get_pool(CAPITAL, avoid, custom_exclude)
    };
    let low_pool = get_pool(LOWER, avoid, custom_exclude);
    let dig_pool = get_pool(DIGITS, avoid, custom_exclude);
    let spec_pool = get_pool(SPECIAL, avoid, custom_exclude);

    let mut password: Vec<char> = Vec::new();
    // Pick mandatory chars ONLY if pools are not empty
    if let Some(c) = low_pool.choose(rng) {
        password.push(*c as char);
    }
    if let Some(c) = dig_pool.choose(rng) {
        password.push(*c as char);
    }
    if let Some(c) = spec_pool.choose(rng) {
        password.push(*c as char);
    }
    if let Some(c) = cap_pool.choose(rng) {
        password.push(*c as char);
    }

    let all_chars: Vec<u8> =
        [&cap_pool[..], &low_pool[..], &dig_pool[..], &spec_pool[..]].concat();

    // The only hard failure: nothing left to use
    if all_chars.is_empty() {
        return "!!!_POOL_EMPTY_!!!".to_string();
    }

    let remaining = len.saturating_sub(password.len());
    password.extend(
        (0..remaining)
            .map(|_| *all_chars.choose(rng).expect("Pool is empty") as char),
    );

    password.shuffle(rng);
    password.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pool_remove_chars() {
        let base = b"abcdef";
        let exclude = Some("ace".to_string());
        let pool = get_pool(base, false, &exclude);

        let result_str = String::from_utf8(pool).unwrap();
        assert_eq!(result_str, "bdf");
        assert!(!result_str.contains('a'));
        assert!(!result_str.contains('c'));
    }

    #[test]
    fn test_get_pool_ambiguous() {
        let base = b"a01lI";
        // pass 'true' for 'avoid' (the --ambiguous flag)
        let pool = get_pool(base, true, &None);

        let result_str = String::from_utf8(pool).unwrap();
        // should only keep 'a', removing 0, 1, l, I
        assert_eq!(result_str, "a");
    }

    #[test]
    fn test_no_capitalize_logic() {
        let mut rng = rand::rng();
        // generate a long password to increase statistical certainty
        let pwd = generate_secure_password(&mut rng, 100, false, true, &None);

        // check that no character is uppercase
        assert!(pwd.chars().all(|c| !c.is_uppercase()));
    }

    #[test]
    fn test_pool_empty_fallback() {
        let mut rng = rand::rng();
        // exclude everything
        let exclude = Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?/`~'\"\\".to_string());
        let pwd =
            generate_secure_password(&mut rng, 12, false, false, &exclude);

        assert_eq!(pwd, "!!!_POOL_EMPTY_!!!");
    }
}
