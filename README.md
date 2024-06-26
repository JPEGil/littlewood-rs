# Littlewood-RS
A CLI app written in Rust that generates Littlewood polynomials, finds their complex roots, and plots them in the complex-plane.
## TODO List
- [x] ~~Create executable that generates Littlewood polynomials.~~
- [x] ~~Create an executable that generates the roots of those polynomials.~~
- [x] ~~Create an executable that plots those roots and returns an image of the fractal.~~
- [x] ~~Create an executable that does all 3 in 1 go.~~
- [ ] Add a progress bar while things generate
- [ ] Fix zoom in plot-complex
- [ ] Speed up program
## Usage
There are 4 executables, generate-littlewood, root-complex, plot-complex, and littlewood-rs. littlewood-rs is just a combined executable that does the job of the other 3 executables at the same time.
### generate-littlewood
```bash
Usage: generate-littlewood[EXE] [OPTIONS]

Options:
  -f, --file-path <FILE_PATH>  Path of Polynomial file [default: ./Polynomials.json]
  -d, --degree <DEGREE>        Highest degree of polynomials [default: 10]
  -H, --help                   Print help
  -V, --version                Print version
```
### root-complex
```bash
Usage: root-complex[EXE] [OPTIONS]

Options:
  -i, --in-file-path <IN_FILE_PATH>    Path of Polynomial file [default: ./Polynomials.json]
  -o, --out-file-path <OUT_FILE_PATH>  Path of Roots file [default: ./Roots.json]
  -H, --help                           Print help
  -V, --version                        Print version
```
### plot-complex
```bash
Usage: plot-complex[EXE] [OPTIONS]

Options:
  -r, --roots-path <ROOTS_PATH>        Path of Roots file [default: ./Roots.json]
  -o, --out-file-path <OUT_FILE_PATH>  Path of Fractal Image file [default: ./Littlewood-Fractal.png]
  -w, --width <WIDTH>                  Width of image [default: 800]
  -h, --height <HEIGHT>                Height of image [default: 600]
  -H  --help                           Print help
  -V, --version                        Print version
```
### littlewood-rs
```bash
Usage: littlewood-rs[EXE] [OPTIONS]

Options:
  -d, --degree <DEGREE>                Highest degree of polynomials [default: 10]
  -o, --out-file-path <OUT_FILE_PATH>  Path of Fractal Image file [default: ./Littlewood-Fractal.png]
  -w, --width <WIDTH>                  Width of image [default: 8000]
  -h, --height <HEIGHT>                Height of image [default: 6000]
  -H, --help                           Print help
  -V, --version                        Print version
```
## Building
So far I have only been able to test on Windows 10/11.
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