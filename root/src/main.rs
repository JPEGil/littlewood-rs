use serde_json;
use clap::Parser;
use std::fs;
use polynomen::Poly;

#[derive(Debug, Parser)]
#[command(version, about, long_about= None)]
struct Args{
    ///Path of Polynomial file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Polynomials.json".to_owned()))]
    in_file_path: String,

    ///Path of Polynomial file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Roots.json".to_owned()))]
    out_file_path: String,
}

fn main() {
    let args = Args::parse();

    let poly_str = fs::read_to_string(&args.in_file_path).unwrap();
    let str = poly_str.as_str();
    let polys = serde_json::from_str::<Vec<Poly<f64>>>(str).unwrap();
}
