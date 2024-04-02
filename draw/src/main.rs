use clap::Parser;
use serde_json;
use std::fs;
use plot_complex::plot_image;

#[derive(Debug, Parser)]
#[command(version, about, long_about= None)]
struct Args{
    ///Path of Roots file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Roots.json".to_owned()))]
    roots_path: String,

    ///Path of Fractal Image file
    // Doesn't work without extra parens. DO NOT REMOVE EXTRA PARENTHESISES
    #[arg(short, long, default_value_t=("./Littlewood-Fractal.png".to_owned()))]
    out_file_path: String,

    ///Width of image
    #[arg(short, long, default_value_t=8000)]
    width: u32,

    ///Height of image
    #[arg(short, long, default_value_t=6000)]
    height: u32,
}
fn main() {
    let args = Args::parse();
    let roots_str = fs::read_to_string(&args.roots_path).unwrap();
    let str = roots_str.as_str();
    let roots = serde_json::from_str::<Vec<(f64, f64)>>(str).unwrap();
    // println!("{roots:?}");
    let buf = plot_image(&roots, args.width, args.height);

    let _ = buf.save(args.out_file_path);
}
