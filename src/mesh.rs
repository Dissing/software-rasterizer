use std::fs::File;
use std::io::Read;
use crate::vectors::{Vec2, Vec3, Vec4};

pub struct Mesh {
    pub vertices: Vec<Vec4>,
    pub tex_coords: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub stride: usize,
}

fn parse_face(parts: Vec<&str>, indices: &mut Vec<u32>)  {
    let mut v: Vec<Vec<&str>> = Vec::new();
    for i in 0..3 {
        v.push(parts[i+1].split("/").collect());
    }

    for i in 0..3 {
        for j in 0..3 {
            indices.push(v[j][i].parse::<u32>().unwrap())
        }
    }
}

pub fn load(name: &str) -> std::io::Result<Mesh> {
    let mut content = String::new();
    let mut file = File::open(name)?;
    file.read_to_string(&mut content);

    let mut vertices = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "v" {
            vertices.push( Vec4 {
                x: parts[1].parse::<f64>().unwrap(),
                y: parts[2].parse::<f64>().unwrap(),
                z: parts[3].parse::<f64>().unwrap(),
                w: 1.0
            });
        } else if parts[0] == "vn" {
            normals.push( Vec3 {
                x: parts[2].parse::<f64>().unwrap(),
                y: parts[3].parse::<f64>().unwrap(),
                z: parts[4].parse::<f64>().unwrap(),
            });
        } else if parts[0] == "vt" {
            tex_coords.push( Vec2 {
                x: parts[2].parse::<f64>().unwrap(),
                y: parts[3].parse::<f64>().unwrap(),
            });
        } else if parts[0] == "f" {
            parse_face(parts, &mut indices);
        } else {
            //Skip
        }
    }

    Ok(Mesh {vertices, indices, tex_coords, normals, stride: 3+3+3 })
}