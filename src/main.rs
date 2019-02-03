use std::usize;

mod tga;
mod mesh;
mod vectors;

use crate::vectors::{Vec4, Mat4};


fn triangle(v0: Vec4, v1: Vec4, v2: Vec4, color: tga::Color, img: &mut tga::Image) {

    line(v0.x, v0.y, v1.x, v1.y, color, img);
    line(v1.x, v1.y, v2.x, v2.y, color, img);
    line(v2.x, v2.y, v0.x, v0.y, color, img);

}

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
        if steep {
            img.set(y as usize, x as usize, color);
        } else {
            img.set(x as usize, y as usize, color);
        }
    }
}

fn main() {
    let mesh = mesh::load("obj/african_head/african_head.obj").unwrap();
    let mut img = tga::create(1024, 1024);

    let w = img.width as f64;
    let h = img.height as f64;

    let cam_mat = Mat4::new(
        w/2.0, 0.0, 0.0, w/2.0,
        0.0, h/2.0, 0.0, h/2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    for i in (0..mesh.indices.len()).step_by(3) {

        let v0 = cam_mat * mesh.vertices[(mesh.indices[i] - 1) as usize];
        let v1 = cam_mat * mesh.vertices[(mesh.indices[i + 1] - 1) as usize];
        let v2 = cam_mat * mesh.vertices[(mesh.indices[i + 2] - 1) as usize];

        triangle(v0, v1, v2, tga::Color {r: 255, g: 0, b: 0}, &mut img);

    }

    line(128.0,128.0, 896.0, 128.0,tga::Color {r: 255, g: 0, b: 0}, &mut img);
    line(896.0,128.0, 896.0, 896.0,tga::Color {r: 0, g: 255, b: 0}, &mut img);
    line(896.0,896.0, 128.0, 896.0,tga::Color {r: 0, g: 0, b: 255}, &mut img);
    line(128.0,896.0, 128.0, 128.0,tga::Color {r: 255, g: 255, b: 255}, &mut img);

    tga::save(img, "output.tga").unwrap();
}
