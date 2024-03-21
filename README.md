# Littlewood-RS
A CLI app written in Rust that generates Littlewood polynomials, finds their complex roots, and plots them in the complex-plane.
## TODO List
- [x] Create executable that generates Littlewood polynomials.
- [ ] Create an executable that generates the roots of those polynomials.
- [ ] Create an executable that plots those roots and returns an image of the fractal.
- [ ] Create an executable that does all 3 in 1 go.
- [ ] Add a progress bar while things generate
## Usage
There are 4 executables, generate-littlewood, root-complex, plot-complex, and littlewood-rs. littlewood-rs is just a combined executable that does ther job of the other 3 executables at the same time.
### generate-littlewood
```bash
Usage: generate-littlewood[EXE] [OPTIONS]

Options:
  -f, --file-path <FILE_PATH>  Path of Polynomial file [default: ./Polynomials.json]
  -d, --degree <DEGREE>        Highest degree of polynomials [default: 10]
  -h, --help                   Print help
  -V, --version                Print version
```
### root-complex
TODO
### plot-complex
TODO
### littlewood-rs
TODO
## Building
So far I have only been able to test on Windows 11.
### Prerequisites
- Rust 2021
- Cargo
---
1. Clone this repository and cd into the root.
2. Run 
```bash
cargo build --release
```
Once finished, all 4 executable should be in ./target/release