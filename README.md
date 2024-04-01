# Littlewood-RS
A CLI app written in Rust that generates Littlewood polynomials, finds their complex roots, and plots them in the complex-plane.
## TODO List
- [x] Create executable that generates Littlewood polynomials.
- [x] Create an executable that generates the roots of those polynomials.
- [ ] Create an executable that plots those roots and returns an image of the fractal.
- [ ] Create an executable that does all 3 in 1 go.
- [ ] Add a progress bar while things generate
## Usage
There are 4 executables, generate-littlewood, root-complex, plot-complex, and littlewood-rs. littlewood-rs is just a combined executable that does the job of the other 3 executables at the same time.
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
```bash
Usage: root-complex[EXE] [OPTIONS]

Options:
  -i, --in-file-path <IN_FILE_PATH>    Path of Polynomial file [default: ./Polynomials.json]
  -o, --out-file-path <OUT_FILE_PATH>  Path of Roots file [default: ./Roots.json]
  -h, --help                           Print help
  -V, --version                        Print version
```
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
1. Clone this repository and cd into the folder of the module you want to build (project root is littlewood-rs)
2. Run 
```bash
cargo build --release
```
Once finished, the executable should be in ./target/release