use polynomen::Poly as AutoPoly;
use indicatif::{ProgressStyle, ParallelProgressIterator};
use rayon::prelude::*;

pub fn roots(polys: &Vec<AutoPoly<f64>>, no_real: bool) -> Vec<(f64, f64)> {

    let mut vec: Vec<Vec<(f64, f64)>> = vec![Vec::new(); polys.len()];

    let style = ProgressStyle::with_template("{wide_bar}{percent}% ETA:{eta}").ok().unwrap();

    vec.par_iter_mut().enumerate().progress_with_style(style).for_each_with(polys, |p, (i, v)|{
        let mut roots = p[i].complex_roots();

        if no_real{
            let mut i:usize = 0;
            while i < roots.len(){
                if roots[i].1 == 0.0{
                    roots.remove(i);
                    continue;
                }
                i += 1;
            }
        }

        *v = roots;
    });

    vec.into_iter().flatten().collect()
}
