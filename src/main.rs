use clap::Parser;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
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

    // Build character pools once, not per-password
    let cap_pool = if args.no_capitalize || args.digit {
        vec![]
    } else {
        get_pool(CAPITAL, args.safe, args.remove_chars.as_deref())
    };
    let low_pool = if args.digit {
        vec![]
    } else {
        get_pool(LOWER, args.safe, args.remove_chars.as_deref())
    };
    let spec_pool = if args.digit {
        vec![]
    } else {
        get_pool(SPECIAL, args.safe, args.remove_chars.as_deref())
    };
    let dig_pool = get_pool(DIGITS, args.safe, args.remove_chars.as_deref());

    for i in 0..num_to_generate {
        let output: String = generate_secure_password(
            &mut rng, final_len, &cap_pool, &low_pool, &dig_pool, &spec_pool,
        );

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

/// Helper to filter out ambiguous characters if requested
fn get_pool(base: &[u8], avoid: bool, custom_exclude: Option<&str>) -> Vec<u8> {
    base.iter()
        .filter(|&&c| {
            if avoid && AMBIGUOUS.contains(&c) {
                return false;
            }
            if let Some(exclude) = custom_exclude
                && exclude.as_bytes().contains(&c)
            {
                return false;
            }
            true
        })
        .cloned()
        .collect()
}

/// Generates a password from pre-built character pools.
///
/// Pools are passed as slices to avoid per-call allocations.
/// Empty slices are treated as disabled pools.
fn generate_secure_password(
    rng: &mut impl Rng,
    len: usize,
    cap_pool: &[u8],
    low_pool: &[u8],
    dig_pool: &[u8],
    spec_pool: &[u8],
) -> String {
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

    let all_chars: Vec<u8> = [cap_pool, low_pool, dig_pool, spec_pool].concat();

    // The only hard failure: nothing left to use
    if all_chars.is_empty() {
        return "!!!_POOL_EMPTY_!!!".to_string();
    }

    let remaining = len.saturating_sub(password.len());
    password.extend(
        (0..remaining)
            .filter_map(|_| all_chars.choose(rng).map(|c| *c as char)),
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
        let pool = get_pool(base, false, exclude.as_deref());

        let result_str = String::from_utf8(pool).unwrap();
        assert_eq!(result_str, "bdf");
        assert!(!result_str.contains('a'));
        assert!(!result_str.contains('c'));
    }

    #[test]
    fn test_get_pool_ambiguous() {
        let base = b"a01lI";
        // pass 'true' for 'avoid' (the --ambiguous flag)
        let pool = get_pool(base, true, None);

        let result_str = String::from_utf8(pool).unwrap();
        // should only keep 'a', removing 0, 1, l, I
        assert_eq!(result_str, "a");
    }

    #[test]
    fn test_no_capitalize_logic() {
        let mut rng = rand::rng();
        // Build pools with no caps enabled
        let cap_pool: Vec<u8> = vec![];
        let low_pool = get_pool(LOWER, false, None);
        let dig_pool = get_pool(DIGITS, false, None);
        let spec_pool = get_pool(SPECIAL, false, None);

        // generate a long password to increase statistical certainty
        let pwd = generate_secure_password(
            &mut rng, 100, &cap_pool, &low_pool, &dig_pool, &spec_pool,
        );

        // check that no character is uppercase
        assert!(pwd.chars().all(|c| !c.is_uppercase()));
    }

    #[test]
    fn test_pool_empty_fallback() {
        let mut rng = rand::rng();
        // Build empty pools (everything excluded)
        let cap_pool: Vec<u8> = vec![];
        let low_pool: Vec<u8> = vec![];
        let dig_pool: Vec<u8> = vec![];
        let spec_pool: Vec<u8> = vec![];

        let pwd = generate_secure_password(
            &mut rng, 12, &cap_pool, &low_pool, &dig_pool, &spec_pool,
        );

        assert_eq!(pwd, "!!!_POOL_EMPTY_!!!");
    }
}
