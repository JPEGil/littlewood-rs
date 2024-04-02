use image::{RgbaImage as Buf, Rgba};

pub fn plot_image(roots: &Vec<(f64,f64)>, w: u32, h: u32) -> Buf{
    let dim = (w, h);

    let mut buf = Buf::new(dim.0, dim.1);

    // let scalex = 2.0 / dim.0 as f64;
    // let scaley = 2.0 / dim.1 as f64;

    // for (_,_,p) in buf.enumerate_pixels_mut(){
    //     *p = Rgba::<u8>([0,0,0,255]);
    // }

    for point in roots{
        let pixel = buf.get_pixel(point.0 as u32, point.1 as u32);
        let r = pixel.0[0];
        let a = pixel.0[3];

        let new_a: u8;

        if a != 200{
            if r == 255{
                new_a = a + 100;
            }
            else{
                new_a = 100;
            }
        }
        else{
            new_a = 255;
        }

        buf.put_pixel(((point.0 * 2000.0 + dim.0 as f64/2.0)) as u32, ((point.1 * 2000.0 + dim.1 as f64/2.0)) as u32, Rgba::<u8>([255, 0 , 0, 255]));
    }

    buf
}