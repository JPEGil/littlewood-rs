use clap::Parser;
use generate_littlewood::generate::gen_littlewood;
use std::fs;

#[derive(Debug, Parser)]
#[command(version, about, long_about= None)]
struct Args{
    ///Path of Polynomial file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Polynomials.json".to_owned()))]
    file_path: String,

    ///Highest degree of polynomials
    #[arg(short, long, default_value_t=10)]
    degree: u128
}
fn main() {
    let args = Args::parse();

    let str = gen_littlewood(args.degree);
    let _ = fs::write(args.file_path, str);
}
