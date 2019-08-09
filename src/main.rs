use std::usize;
use std::f64;
use std::cmp;

mod tga;
mod mesh;
mod vectors;

use crate::vectors::{Vec2, Vec3, Vec4, Mat4};


fn rasterize<F>(v0: Vec4, v1: Vec4, v2: Vec4, img: &mut tga::Image, z_buffer: &mut [f64], fragment_shader: F) where F: Fn(f64, f64, f64) -> tga::Color {

    fn edge(p: Vec2, e0: Vec4, e1: Vec4) -> f64 {
        (p.x as f64 - e0.x) * (e1.y - e0.y) - (p.y as f64 - e0.y) * (e1.x - e0.x)
    }

    let min_x = v0.x.min(v1.x.min(v2.x)).max(0.0) as usize;
    let min_y = v0.y.min(v1.y.min(v2.y)).max(0.0) as usize;
    let max_x = v0.x.max(v1.x.max(v2.x)).min(img.width as f64 - 1.0) as usize;
    let max_y = v0.y.max(v1.y.max(v2.y)).min(img.height as f64 - 1.0) as usize;

    for i in min_x..max_x+1 {
        for j in min_y..max_y+1 {

            let p = Vec2::new(i as f64, j as f64);
            let a = edge(p, v1, v2);
            let b = edge(p, v2, v0);
            let c = edge(p, v0, v1);

            if a <= 0.0 && b <= 0.0 && c <= 0.0 {

                let area = edge(v0.xy(), v1, v2);
                let w0 = a / area;
                let w1 = b / area;
                let w2 = c / area;

                let z = 1.0/(w0 * 1.0/v0.z + w1 * 1.0/v1.z + w2 * 1.0/v2.z);

                if z < z_buffer[j * img.width + i] {
                    continue;
                }

                let color = fragment_shader(w0, w1, w2);
                img.set(i, j, color);
                z_buffer[j * img.width + i] = z;
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
    let diffuse = tga::load("obj/african_head/african_head_diffuse.tga").unwrap();
    let mut img = tga::create(1024, 1024);

    let mut z_buffer = vec![f64::NEG_INFINITY; img.width * img.height];

    let w = img.width as f64;
    let h = img.height as f64;

    /*rasterize(Vec4::new(10.0, 70.0,0.0,0.0), Vec4::new(50.0,160.0,0.0,0.0),
             Vec4::new(70.0,80.0,0.0,0.0), &mut img, &mut z_buffer);

    rasterize(Vec4::new(180.0, 50.0,0.0,0.0), Vec4::new(150.0,1.0,0.0,0.0),
             Vec4::new(70.0,180.0,0.0,0.0), &mut img, &mut z_buffer);

    rasterize(Vec4::new(180.0, 150.0,0.0,0.0), Vec4::new(120.0,160.0,0.0,0.0),
             Vec4::new(130.0,180.0,0.0,0.0), &mut img, &mut z_buffer);*/

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

        let t0 = mesh.tex_coords[(mesh.indices[i + 3] - 1)  as usize];
        let t1 = mesh.tex_coords[(mesh.indices[i + 4] - 1)  as usize];
        let t2 = mesh.tex_coords[(mesh.indices[i + 5] - 1)  as usize];

        let n0 = mesh.normals[(mesh.indices[i + 6] - 1)  as usize];
        let n1 = mesh.normals[(mesh.indices[i + 7] - 1)  as usize];
        let n2 = mesh.normals[(mesh.indices[i + 8] - 1)  as usize];

        let e0 = v1 - v0;
        let e1 = v2 - v0;

        let face_normal = e0.xyz().cross(e1.xyz()).normalize();
        let intensity = face_normal * light_dir;

        let shader = |a,b,c| {

            let normal: Vec3 = n0 * a + n1 * b + n2 * c;
            let n = normal.normalize();

            let tex: Vec2 = t0 * a + t1 * b + t2 * c;
            let tex_x = (tex.x * diffuse.height as f64) as usize;
            let tex_y = (tex.y * diffuse.height as f64) as usize;
            let d = diffuse.read(tex_x, tex_y);
            if intensity > 0.0 {
                d * intensity
            } else {
                d * 0.01
            }
        };

        if intensity > 0.0 {
            rasterize(cam_mat * v0, cam_mat * v1, cam_mat * v2, &mut img, &mut z_buffer, shader);
        }

    }
    tga::save(img, "output.tga").unwrap();
}
