pub mod generate{
    use poly::poly::*;

    pub fn gen_littlewood(degree: u128) -> Vec<Poly<i8>>{

        let init = Poly{c: vec!(-1; degree as usize)};
        let mut polys:Vec<Poly<i8>> = vec![init];

        let max = 2_u128.pow(degree as u32) as usize;
        let mut i = 1;

        while i < max {
            polys.push(polys[i-1].next());
            i += 1;
        }

        polys
    }
}