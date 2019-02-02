use std::fs::File;
use std::io::Read;

pub struct Mesh {
    pub vertices: Vec<f64>,
    pub tex_coords: Vec<u32>,
    pub tangents: Vec<u32>,
    pub indices: Vec<u32>,
}

fn parse_face_vertex(s: &str, vertices: &mut Vec<u32>, tex_coords: &mut Vec<u32>, tangents: &mut Vec<u32>) {
    let parts: Vec<&str> = s.split("/").collect();
    vertices.push(parts[0].parse::<u32>().unwrap());
    tex_coords.push(parts[1].parse::<u32>().unwrap());
    tangents.push(parts[2].parse::<u32>().unwrap());
}

pub fn load(name: &str) -> std::io::Result<Mesh> {
    let mut content = String::new();
    let mut file = File::open(name)?;
    file.read_to_string(&mut content);

    let mut vertices = Vec::new();
    let mut tex_coords = Vec::new();
    let mut tangents = Vec::new();
    let mut indices = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "v" {
            vertices.push(parts[1].parse::<f64>().unwrap());
            vertices.push(parts[2].parse::<f64>().unwrap());
            vertices.push(parts[3].parse::<f64>().unwrap());
        } else if parts[0] == "vn" {
            //TODO
        } else if parts[0] == "vt" {
            //TODO
        } else if parts[0] == "f" {
            parse_face_vertex(parts[1], &mut indices, &mut tex_coords, &mut tangents);
            parse_face_vertex(parts[2], &mut indices, &mut tex_coords, &mut tangents);
            parse_face_vertex(parts[3], &mut indices, &mut tex_coords, &mut tangents);
        } else {
            //Skip
        }
    }

    Ok(Mesh {vertices, indices, tex_coords, tangents })
}