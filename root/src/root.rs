use polynomen::Poly as AutoPoly;

pub fn roots(polys: &Vec<AutoPoly<f64>>) -> Vec<(f64, f64)> {
    let mut vec: Vec<(f64, f64)> = vec![];
    for p in polys {
        let roots = p.complex_roots();
        for t in roots{
            vec.push(t);
        }
    }
    vec
}