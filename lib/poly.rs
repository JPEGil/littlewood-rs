use serde::{Serialize, Deserialize};
use polynomen::Poly as AutoPoly;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Poly{
    pub num_rep: u128,
    pub degree: u128
}

impl Poly{
    pub fn next(self: &Self) -> Poly{
        Poly{num_rep: self.num_rep + 1, degree: self.degree}
    }
}

impl From<Poly> for AutoPoly<f64>{
    fn from(poly: Poly) -> Self{
        let mut vec: Vec<f64> = vec![];
        for i in 0..poly.degree{
            let bit = (poly.num_rep >> i) & 1;
            match bit{
                0=>vec.push(-1.0),
                1=> vec.push(1.0),
                _ => println!("The world is over. Binary is no longer binary.")
            }
        }
        AutoPoly::new_from_coeffs_iter(vec.into_iter())
    }
}