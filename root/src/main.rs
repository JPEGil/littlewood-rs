use serde_json;
use clap::Parser;
use std::fs;
use polynomen::Poly as AutoPoly;
use poly::Poly;
use root_complex::roots;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None, disable_help_flag = true)]
struct Args{
    ///Path of Polynomial file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Polynomials.json".to_owned()))]
    in_file_path: String,

    ///Path of Roots file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Roots.json".to_owned()))]
    out_file_path: String,

    ///Throws out real number roots
    #[arg(short, long, default_value_t=false)]
    no_real: bool,

    ///Print help
    #[arg(short='H', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let poly_str = fs::read_to_string(&args.in_file_path).unwrap();
    let str = poly_str.as_str();
    let polys = serde_json::from_str::<Vec<Poly>>(str).unwrap();
    let mut auto_polys: Vec<AutoPoly<f64>> = vec![];
    for p in polys{
        let converted:AutoPoly<f64> = AutoPoly::<f64>::from(p);
        auto_polys.push(converted);
    }

    let roots = roots(&auto_polys, args.no_real);
    let str = serde_json::to_string(&roots).ok().unwrap();
    let _ = fs::write(args.out_file_path, str);
}
