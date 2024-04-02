use generate_littlewood::gen_littlewood;
use root_complex::roots;
use plot_complex::plot_image;
use clap::Parser;
use polynomen::Poly as AutoPoly;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None, disable_help_flag = true)]
struct Args{
    ///Highest degree of polynomials
    #[arg(short, long, default_value_t=10)]
    degree: u128,

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

    ///Print help
    #[arg(short = 'H', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let polys = gen_littlewood(args.degree);

    let mut auto_polys:Vec<AutoPoly<f64>> = vec![]; 
    for p in polys{
        let converted:AutoPoly<f64> = AutoPoly::<f64>::from(p);
        auto_polys.push(converted);
    }
    let roots = roots(&auto_polys);
    let image = plot_image(&roots, args.width, args.height);

    let _ = image.save(args.out_file_path);
}
