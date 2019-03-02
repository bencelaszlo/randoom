# randoom
An easy-to-use CLI tool for generating random numbers, texts, and colors.

## Features

- generate random numbers
  - generate floating point numbers
    - with single precision: `$ randoom -t f32 -n <number>`
    - with double precision: `$ randoom -t f64 -n <number>`
  - generate integers
    - `$ randoom -t <i32 / i64 / isize> -n <number`
    - sample input: `-52`
  - generate unsinged integers
    - `$ randoom -t <u32 / u64 / usize> -n <number>`
    - sample input: `23`
- ~~generate random texts~~ (W.I.P.)
- generate random colors
  - generate RGB colors
    - `$ randoom -t rgb -n <number>`
    - sample output: `#ff2353`

## Manual

- Type `$ randoom -t <datatype> -n <number>`, where <datatype> currently can be f32, f64, i32, i64, isize, u32, u64, usize, or rgb
- Type `$ randoom --help` for optional parameters or generic help.

## Build from source

1. Clone this repository: `$ git clone https://github.com/bencelaszlo/randoom.git`
2. Download dependencies and build with Cargo: `$ cargo build --release`
3. Copy the compiled binary to /usr/bin: `# cp target/release/randoom /usr/bin/`

### Dependencies

- Rustup (the Rust installer and version management tool): https://www.rust-lang.org/learn/get-started
- clap (for argument parsing): https://crates.io/crates/clap
- colored (for colored terminal output): https://crates.io/crates/colored

