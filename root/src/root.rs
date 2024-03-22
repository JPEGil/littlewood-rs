use poly::{PosONeg};
use polynomen::Poly;

pub fn roots(polys: &Vec<Poly<f64>>){
    for p in polys {
        p.complex_roots();
    }
}