use std::usize;

mod tga;

fn line(x0: f64, y0: f64, x1: f64, y1: f64, color: tga::Color, img: &mut tga::Image) {

    let mut x0 = x0 as i64;
    let mut x1 = x1 as i64;
    let mut y0 = y0 as i64;
    let mut y1 = y1 as i64;

    let delta_x = x1-x0;
    let delta_y = y1-y0;

    let mut steep = false;

    if delta_x.abs() < delta_y.abs() {
        steep = true;
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    for x in x0..x1 {
        let t = (x-x0) as f64 / (x1-x0) as f64;
        let y = y1 as f64 * t + (y0 as f64 * (1.0-t));
        if (steep) {
            img.set(y as usize, x as usize, color);
        } else {
            img.set(x as usize, y as usize, color);
        }
    }
}

fn main() {
    let mut img = tga::create(1024, 1024);

    line(128.0,128.0, 896.0, 128.0,tga::Color {r: 255, g: 0, b: 0}, &mut img);
    line(896.0,128.0, 896.0, 896.0,tga::Color {r: 0, g: 255, b: 0}, &mut img);
    line(896.0,896.0, 128.0, 896.0,tga::Color {r: 0, g: 0, b: 255}, &mut img);
    line(128.0,896.0, 128.0, 128.0,tga::Color {r: 255, g: 255, b: 255}, &mut img);

    tga::save(img, "output.tga").unwrap();
}
