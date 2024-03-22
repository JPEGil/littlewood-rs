pub mod poly{

    use num_traits::Num;
    //use std::fmt::Display;
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Poly<T: Num>{
        pub c: Vec<T>
    }

    pub trait PosONeg{
        fn next(self: &Self) -> Poly<i8>;
    }

    impl PosONeg for Poly<i8>{
        fn next(self: &Self) -> Poly<i8>{
            let mut i = 1;
            let mut clone = self.c.clone();
            while i <= self.c.len(){
                if self.c[self.c.len() - i] == 1_i8 {
                    clone[self.c.len() - i] = -1;
                }
                else{
                    clone[self.c.len() - i] = 1;
                    break;
                }
                i += 1;
            }
            Poly{c: clone}
        }
    }

    // impl<T: Num + Display> ToString for Poly<T>{
    //     fn to_string(self: &Self) -> String
    //     {
    //         let mut i = self.coefs.len();
    //         let mut s: String = "".to_owned();
    //         while i > 0{
    //             let pow = i - 1;
    //             let num = &self.coefs[pow];
    //             if num.is_zero() {continue;}
    //             let have_x = if pow != 0 {format!("x^{pow}")} else{"".to_owned()};
    //             s += &format!("{num}{have_x}");
    //             i -= 1;
    //         }
    //         s
    //     }
    // }
}