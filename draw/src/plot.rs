use image::{RgbaImage as Buf, Rgba};

pub fn plot_image(roots: &Vec<(f64,f64)>, w: u32, h: u32) -> Buf{
    let dim = (w, h);

    let mut buf = Buf::new(dim.0, dim.1);

    for point in roots{
        buf.put_pixel(((point.0 * 2000.0 + dim.0 as f64/2.0)) as u32, ((point.1 * 2000.0 + dim.1 as f64/2.0)) as u32, Rgba::<u8>([255, 0 , 0, 255]));
    }

    buf
}