use poly::Poly;

pub fn gen_littlewood(degree: u128) -> Vec<Poly>{

    // let init = Poly::new_from_coeffs(&vec!(-1; degree as usize));
    //let init = Poly{c: };
    let mut polys:Vec<Poly> = vec![Poly{num_rep: 0, degree}];

    let max = 2_u128.pow(degree as u32) as usize;
    let mut i = 1;

    while i < max {
        polys.push(polys[i-1].next());
        i += 1;
    }

    polys
}