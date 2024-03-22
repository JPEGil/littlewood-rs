use clap::Parser;
use generate_littlewood::gen_littlewood;
use std::fs;
use serde_json;

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

    let polys = gen_littlewood(args.degree);
    let str = serde_json::to_string_pretty(&polys).ok().unwrap();
    let _ = fs::write(args.file_path, str);
}
