<div align="center">

# Signum

Signum is a lightweight command-line password generator written in Rust.
Generate random passwords or numeric PINs directly from your terminal, with
options to customize their length, quantity, and character sets.

Inspired by [pwgen](https://github.com/tytso/pwgen).

[![CI](https://github.com/riccione/signum/actions/workflows/ci.yml/badge.svg)](https://github.com/riccione/signum/actions)
[![Release](https://img.shields.io/github/v/release/riccione/signum)](https://github.com/riccione/signum/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

## Features

- Generate multiple passwords in a terminal-friendly grid.
- Customize password length and the number of passwords.
- Generate numeric PINs.
- Exclude uppercase letters or ambiguous characters.
- Remove specific characters from the character set.
- Output passwords in a single column, making it convenient to use with other command-line tools.

## Demo

```console
$ signum 12 6
VLAZS%fK6Npl -i$yqZ#J6rgQ ai4(czTq7)W& LHhKZ#Lq@Nq8 FBEzgb6(BMvX dS5OV#3gUr0e

$ signum -B -1 10
J6*kWxBK95

$ signum 10 3 -r 0OIl1 
E&)9yPubm5 *r&)-dk75P iY%)6AEd^T

$ signum 5 3 -d -r0 
48117 72324 62765 
```

## Install

For Linux, we provide two builds:
* **`linux-musl-x86_64` (Recommended):** Statically linked with musl. Does not require glibc or other shared libraries. 
* **`linux-x86_64`:** Dynamically linked against glibc. Requires a compatible glibc installation.

**Linux (musl-x86_64):**

```sh
curl -LO https://github.com/riccione/signum/releases/latest/download/signum_linux-musl-x86_64
chmod +x signum_linux-musl-x86_64
sudo mv signum_linux-musl-x86_64 /usr/local/bin/signum
```

**Linux (x86_64):**

```sh
curl -LO https://github.com/riccione/signum/releases/latest/download/signum_linux-x86_64
chmod +x signum_linux-x86_64
sudo mv signum_linux-x86_64 /usr/local/bin/signum
```

**macOS:**

```sh
# Apple Silicon
curl -LO https://github.com/riccione/signum/releases/latest/download/signum_macos-arm64
chmod +x signum_macos-arm64
sudo mv signum_macos-arm64 /usr/local/bin/signum

# Intel
curl -LO https://github.com/riccione/signum/releases/latest/download/signum_macos-x86_64
chmod +x signum_macos-x86_64
sudo mv signum_macos-x86_64 /usr/local/bin/signum
```

**Windows:** download `signum_win-x86_64.exe` from the
[releases page](https://github.com/riccione/signum/releases) and put it on
your `PATH`.

Every release also publishes `SHA256SUMS`.

**With Cargo:**

```sh
cargo install --git https://github.com/riccione/signum
```

**From source** (Rust 1.97.1+):

```sh
git clone https://github.com/riccione/signum
cd signum
cargo build --release
```

## Usage

### Generate passwords

Run Signum without arguments to generate a grid of 12-character passwords:

```bash
signum
```

By default, generated passwords contain at least one character from each of these groups:

- Lowercase letters (`a-z`)
- Uppercase letters (`A-Z`)
- Digits (`0-9`)
- Special characters `")-(*&^%$#@!~"`

Signum automatically adjusts the grid layout to your terminal width (up to 80 characters).

### Customize password length and quantity

Signum accepts two positional arguments:

```text
signum [LENGTH] [COUNT]
```

| Argument | Description | Default |
|---|---|---|
| `LENGTH` | Password length | `12` |
| `COUNT` | Number of passwords | `156` |

By default, Signum generates 156 passwords in grid mode and one password in single-column mode.

For example, generate five passwords, each 20 characters long:

```bash
signum 20 5
```

### Generate numeric PINs

Use `-d` or `--digit` to generate numeric PINs. The default PIN length is 5 characters.

```bash
signum -d
```

You can also specify the length and quantity:

```bash
signum -d 6 10
```

### Customize the character set

Signum provides several options to customize generated passwords:

| Option | Description |
|---|---|
| `-d`, `--digit` | Generate numeric PINs (default length: 5). |
| `-1` | Display passwords in a single column. |
| `-A`, `--no-capitalize` | Exclude uppercase letters. |
| `-B`, `--ambiguous` | Exclude ambiguous characters (`O`, `0`, `I`, `l`, `1`). |
| `-r`, `--remove-chars <CHARS>` | Exclude specific characters from the character set. |
| `-h`, `--help` | Display help information. |
| `-V`, `--version` | Display version information. |

Examples:

Generate passwords without uppercase letters:

```bash
signum -A
```

Avoid ambiguous characters:

```bash
signum -B
```

Exclude specific characters:

```bash
signum -r '0O1Il'
```

Generate PINs without the digits `1`, `2`, and `3`:

```bash
signum -d -r 123
```

Generate ten 16-character passwords in a single column:

```bash
signum -1 16 10
```

For a complete list of available options, run:

```bash
signum --help
```

## Development

Build the project:

```bash
cargo build
```

Run the test suite:

```bash
cargo test
```

## Contributing

Contributions, bug reports, and feature suggestions are welcome.

- Open an issue to report a bug or discuss an idea.
- Fork the repository and submit a pull request with your changes.

## License

Signum is distributed under the [MIT License](LICENSE).
