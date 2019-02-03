use std::usize;
use std::cmp;
use std::f64;

mod tga;
mod mesh;
mod vectors;

use crate::vectors::{Vec3, Vec4, Mat4};

fn sort_2_vertices(v0: Vec4, v1: Vec4) -> (Vec4, Vec4) {
    if v0.y <= v1.y {
        (v0, v1)
    } else {
        (v1, v0)
    }
}

fn sort_3_vertices(v0: Vec4, v1: Vec4, v2: Vec4) -> (Vec4, Vec4, Vec4) {
    if v0.y <= v1.y && v0.y <= v2.y {
        let (snd, thd) = sort_2_vertices(v1, v2);
        (v0, snd, thd)
    } else if v1.y <= v0.y && v1.y <= v2.y {
        let (snd, thd) = sort_2_vertices(v0, v2);
        (v1, snd, thd)
    } else {
        let (snd, thd) = sort_2_vertices(v0, v1);
        (v2, snd, thd)
    }
}


fn triangle(v0: Vec4, v1: Vec4, v2: Vec4, color: tga::Color, img: &mut tga::Image, z_buffer: &mut [f64]) {

    let (fst, snd, thd) = sort_3_vertices(v0, v1, v2);

    let fst_y = fst.y as usize;
    let snd_y = snd.y as usize;
    let thd_y = thd.y as usize;

    for y in fst_y..snd_y {
        let t_2 = (y-fst_y) as f64 / (snd_y - fst_y) as f64;
        let t_3 = (y-fst_y) as f64 / (thd_y - fst_y) as f64;
        let x_2 = (t_2*snd.x + (1.0 - t_2) * fst.x) as usize;
        let x_3 = (t_3*thd.x + (1.0 - t_3) * fst.x) as usize;
        let z_2 = (t_2*snd.z + (1.0 - t_2) * fst.z);
        let z_3 = (t_3*thd.z + (1.0 - t_3) * fst.z);

        let (left_x, right_x, left_z, right_z) = if x_2 < x_3 {
            (x_2, x_3, z_2, z_3)
        } else {
            (x_3, x_2, z_3, z_2)
        };

        for x in left_x..right_x {
            let tz = (x-left_x) as f64 / (right_x - left_x) as f64;
            let z = right_z * tz + left_z * (1.0 - tz);
            let zidx = y * img.width + x;
            if z > z_buffer[zidx] {
                z_buffer[zidx] = z;
                img.set(x, y,  color);
            }
        }
    }

    for y in snd_y..thd_y {
        let t_2 = (thd_y - y) as f64 / (thd_y - fst_y) as f64;
        let t_3 = (thd_y - y) as f64 / (thd_y - snd_y) as f64;
        let x_2 = (t_2*fst.x + (1.0 - t_2) * thd.x) as usize;
        let x_3 = (t_3*snd.x + (1.0 - t_3) * thd.x) as usize;
        let z_2 = (t_2*fst.z + (1.0 - t_2) * thd.z);
        let z_3 = (t_3*snd.z + (1.0 - t_3) * thd.z);

        let (left_x, right_x, left_z, right_z) = if x_2 < x_3 {
            (x_2, x_3, z_2, z_3)
        } else {
            (x_3, x_2, z_3, z_2)
        };

        for x in left_x..right_x {
            let tz = (x-left_x) as f64 / (right_x - left_x) as f64;
            let z = right_z * tz + left_z * (1.0 - tz);
            let zidx = y * img.width + x;
            if z > z_buffer[zidx] {
                z_buffer[zidx] = z;
                img.set(x, y,  color);
            }
        }
    }
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

    let mut z_buffer = vec![f64::NEG_INFINITY; 1024*1024];

    let w = img.width as f64;
    let h = img.height as f64;

    /*triangle(Vec4::new(10.0, 70.0,0.0,0.0), Vec4::new(50.0,160.0,0.0,0.0),
             Vec4::new(70.0,80.0,0.0,0.0), tga::Color {r: 255, g: 255, b: 255}, &mut img);

    triangle(Vec4::new(180.0, 50.0,0.0,0.0), Vec4::new(150.0,1.0,0.0,0.0),
             Vec4::new(70.0,180.0,0.0,0.0), tga::Color {r: 255, g: 255, b: 255}, &mut img);

    triangle(Vec4::new(180.0, 150.0,0.0,0.0), Vec4::new(120.0,160.0,0.0,0.0),
             Vec4::new(130.0,180.0,0.0,0.0), tga::Color {r: 255, g: 255, b: 255}, &mut img);*/

    let cam_mat = Mat4::new(
        w/2.0, 0.0, 0.0, w/2.0,
        0.0, h/2.0, 0.0, h/2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let light_dir = Vec3::new(0.0, 0.0, 1.0);

    for i in (0..mesh.indices.len()).step_by(mesh.stride) {

        let v0 = mesh.vertices[(mesh.indices[i] - 1) as usize];
        let v1 = mesh.vertices[(mesh.indices[i + 1] - 1) as usize];
        let v2 = mesh.vertices[(mesh.indices[i + 2] - 1) as usize];

        let _t0 = mesh.tex_coords[(mesh.indices[i + 3] - 1)  as usize];
        let _t1 = mesh.tex_coords[(mesh.indices[i + 4] - 1)  as usize];
        let _t2 = mesh.tex_coords[(mesh.indices[i + 5] - 1)  as usize];

        let n0 = mesh.normals[(mesh.indices[i + 6] - 1)  as usize];
        let n1 = mesh.normals[(mesh.indices[i + 7] - 1)  as usize];
        let n2 = mesh.normals[(mesh.indices[i + 8] - 1)  as usize];

        let e0 = v1 - v0;
        let e1 = v2 - v0;

        let face_normal = e0.xyz().cross(e1.xyz()).normalize();


        let intensity = face_normal * light_dir;

        if intensity > 0.0 {
            triangle(cam_mat * v0, cam_mat * v1, cam_mat * v2,
                     tga::Color {r: 255, g: 255, b: 255} * intensity, &mut img, &mut z_buffer);
        }

    }

    tga::save(img, "output.tga").unwrap();
}
