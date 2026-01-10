<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
<div align="center">
  <h3 align="center">Signum: CLI password generator</h3>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#testing">Testing</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

Simple CLI password generator written in Rust. Mainly for self education and self use.

Build with:
- [Clap](https://crates.io/crates/clap)
- [Rand](https://crates.io/crates/rand)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

* RUST 1.77.0+

### Installation

Build from source:
```
cargo build --release
```

The binary is located in [target/release/signum](https://github.com/riccione/signum/releases).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

Running without arguments generates a grid of **12-character** passwords. By
default, each password contains at least one lowercase letter, one digit, one
special symbol, and one uppercase letter.

### Default Grid Layout

Signum automatically calculates how many passwords fit in your terminal width
(max 80 chars).

```
$ signum
# Generates a grid of 12-char passwords
```
#### Positional Arguments
The length and number of passwords can be set directly without flags:

`$ signum [LEN] [NUM]`

* **LEN** (u8): Define password length (default 12, or 5 for PINs).
* **NUM** (u16): Number of passwords to generate (default 156 for grid).

Flags:

* `-d`, `--digit`: Generate numeric PINs (default length 5).
* `-1`: Force single column output (useful for pipes).
* `-A`, `no-capitalize`: Exclude all uppercase letters.
* `-B`, `--ambiguous`: Avoid confusing characters (O, 0, I, l, 1).
* `-r`, `--remove-chars` [str]: Exclude specific characters from the pool.
  * `$ signum -d -r 123` (Generates a PIN without 1, 2, or 3)
* `-h` or `--help` = help
* `-V` or `--version`

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Testing

`cargo test`

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this better, please fork the repo and
create a pull request. You can also simply open an issue with the tag
"enhancement".  Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

No reason to contact with me ^_-.
Just create an issue if you need something.

Project Link:
[https://github.com/riccionee/hermes](https://github.com/riccione/hermes)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* **Inspired by `pwgen`**
* [Choose an Open Source License](https://choosealicense.com)
* [Rust](https://www.rust-lang.org/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>
