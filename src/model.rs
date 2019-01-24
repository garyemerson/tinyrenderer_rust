use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;
use std::f32;
use nalgebra::geometry::{Point2, Point3};
use nalgebra::core::{Vector3};

pub struct Model {
    // Triangles that make up to the faces of the object
    pub faces: Vec<[Point3<f32>; 3]>,

    // Normal vectors for each vertice of each face
    pub face_normals: Vec<[Vector3<f32>; 3]>,

    // For each face, the 2d points in the texture that correspond to each of face vertices
    pub face_texture_vertices: Vec<[Point2<f32>; 3]>,
}

struct RawModel {
    // Actual points (0.247512 -0.942667 0.275986) that are referenced by 1-based index
    vertices: Vec<(f32, f32, f32)>,

    // Actual points (0.549 0.958 0.000) in the texture that are referenced by 1-based index
    texture_vertices: Vec<(f32, f32)>,

    // First tuple contains indexes into vertices that give the 3 points that make up this
    // face/triangle.
    // Second tuple contains indexes into texture_vertices that describe what parts of the texture
    // fit over this face.
    faces: Vec<((usize, usize, usize), (usize, usize, usize))>,

    // Actual vectors (0.001 0.482 -0.876) that give the normal to the corresponding vertice.
    vertice_normals: Vec<(f32, f32, f32)>
}

impl From<RawModel> for Model {
    fn from(raw_model: RawModel) -> Model {
        let mut faces: Vec<[Point3<f32>; 3]> = Vec::with_capacity(raw_model.faces.len());
        let mut face_normals: Vec<[Vector3<f32>; 3]> = Vec::with_capacity(raw_model.faces.len());
        let mut face_texture_vertices: Vec<[Point2<f32>; 3]> = Vec::with_capacity(raw_model.faces.len());
        for f in raw_model.faces {
            let p0 = Point3::new( 
                raw_model.vertices[((f.0).0 - 1)].0,
                raw_model.vertices[((f.0).0 - 1)].1,
                raw_model.vertices[((f.0).0 - 1)].2);
            let p1 = Point3::new(
                raw_model.vertices[((f.0).1 - 1)].0,
                raw_model.vertices[((f.0).1 - 1)].1,
                raw_model.vertices[((f.0).1 - 1)].2);
            let p2 = Point3::new(
                raw_model.vertices[((f.0).2 - 1)].0,
                raw_model.vertices[((f.0).2 - 1)].1,
                raw_model.vertices[((f.0).2 - 1)].2);
            faces.push([p0, p1, p2]);

            let vn0 = Vector3::new( 
                raw_model.vertice_normals[((f.0).0 - 1)].0,
                raw_model.vertice_normals[((f.0).0 - 1)].1,
                raw_model.vertice_normals[((f.0).0 - 1)].2);
            let vn1 = Vector3::new(
                raw_model.vertice_normals[((f.0).1 - 1)].0,
                raw_model.vertice_normals[((f.0).1 - 1)].1,
                raw_model.vertice_normals[((f.0).1 - 1)].2);
            let vn2 = Vector3::new(
                raw_model.vertice_normals[((f.0).2 - 1)].0,
                raw_model.vertice_normals[((f.0).2 - 1)].1,
                raw_model.vertice_normals[((f.0).2 - 1)].2);
            face_normals.push([vn0, vn1, vn2]);

            let (vt0, vt1, vt2) = ((f.1).0 - 1, (f.1).1 - 1, (f.1).2 - 1);
            let tx0 = Point2::new(raw_model.texture_vertices[vt0].0 * 1024.0, raw_model.texture_vertices[vt0].1 * 1024.0);
            let tx1 = Point2::new(raw_model.texture_vertices[vt1].0 * 1024.0, raw_model.texture_vertices[vt1].1 * 1024.0);
            let tx2 = Point2::new(raw_model.texture_vertices[vt2].0 * 1024.0, raw_model.texture_vertices[vt2].1 * 1024.0);
            face_texture_vertices.push([tx0, tx1, tx2]);
        }

        Model {
            faces: faces,
            face_normals: face_normals,
            face_texture_vertices: face_texture_vertices,
        }
    }
}

pub fn parse_obj_file(path: &str) -> Model {
    let f = File::open(path).unwrap();
    let buffer = BufReader::new(f);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut texture_vertices = Vec::new();
    let mut vertice_normals = Vec::new();

    let vre = Regex::new(r"v\s+([\d\-\.e]+)\s+([\d\-\.e]+)\s+([\d\-\.e]+)").unwrap();
    let fre = Regex::new(r"f\s+(\d*)/(\d*)[^ ]+\s+(\d*)/(\d*)[^ ]+\s+(\d*)/(\d*)[^ ]+").unwrap();
    let tre = Regex::new(r"vt\s+([\d\-\.e]+)\s+([\d\-\.e]+)").unwrap();
    let vnre = Regex::new(r"vn\s+([\d\-\.e]+)\s+([\d\-\.e]+)\s+([\d\-\.e]+)").unwrap();

    for l in buffer.lines() {
        let l = l.unwrap();
        if l.starts_with("v ") {
            for cap in vre.captures_iter(&l) {
                let v1 = &cap[1].trim();
                let v2 = &cap[2].trim();
                let v3 = &cap[3].trim();
                let v = (v1.parse::<f32>().unwrap(), v2.parse::<f32>().unwrap(), v3.parse::<f32>().unwrap());
                vertices.push(v);
            }
        } else if l.starts_with("f ") {
            for cap in fre.captures_iter(&l) {
                let f1 = &cap[1].trim();
                let f2 = &cap[3].trim();
                let f3 = &cap[5].trim();
                let ft1 = &cap[2].trim();
                let ft2 = &cap[4].trim();
                let ft3 = &cap[6].trim();
                let face = (f1.parse::<usize>().unwrap(), f2.parse::<usize>().unwrap(), f3.parse::<usize>().unwrap());
                let face_texture = (ft1.parse::<usize>().unwrap(), ft2.parse::<usize>().unwrap(), ft3.parse::<usize>().unwrap());
                faces.push((face, face_texture));
            }
        } else if l.starts_with("vt ") {
            for cap in tre.captures_iter(&l) {
                let vt1 = &cap[1].trim();
                let vt2 = &cap[2].trim();
                let vt = (vt1.parse::<f32>().unwrap(), vt2.parse::<f32>().unwrap());
                texture_vertices.push(vt);
            }
        } else if l.starts_with("vn ") {
            for cap in vnre.captures_iter(&l) {
                let vn1 = &cap[1].trim();
                let vn2 = &cap[2].trim();
                let vn3 = &cap[3].trim();
                let vn = (vn1.parse::<f32>().unwrap(), vn2.parse::<f32>().unwrap(), vn3.parse::<f32>().unwrap());
                vertice_normals.push(vn);
            }
        }
    }

    Model::from(
        RawModel {
            vertices: vertices,
            faces: faces,
            texture_vertices: texture_vertices,
            vertice_normals: vertice_normals
        })
}
