use num_traits::Num;
use serde::{Serialize, Deserialize};
use polynomen::{Poly};

pub trait PosONeg{
    fn next(self: &Self) -> Poly<f64>;
}

impl PosONeg for Poly<f64>{
    fn next(self: &Self) -> Poly<f64>{
        let mut i = 1;
        let mut clone = self.coeffs();
        let len = clone.len();
        while i <= len{
            if clone[len - i] == 1.0_f64 {
                clone[len - i] = -1.0;
            }
            else{
                clone[len - i] = 1.0;
                break;
            }
            i += 1;
        }
        Poly::new_from_coeffs(&clone)
    }
}
