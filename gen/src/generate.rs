pub mod generate{
    use poly::poly::*;
    use serde_json;

    pub fn gen_littlewood(degree: u128) -> String{

        let init = Poly{c: vec!(-1; degree as usize)};
        let mut polys:Vec<Poly<i8>> = vec![init];

        let max = 2_u128.pow(degree as u32) as usize;
        let mut i = 1;

        //let mut str = "".to_owned();

        while i < max {
            polys.push(polys[i-1].next());
            i += 1;
        }

        serde_json::to_string(&polys).ok().unwrap()
    }
}