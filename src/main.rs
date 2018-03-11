extern crate regex;
extern crate image;
extern crate rand;
extern crate nalgebra;
extern crate cgmath;

mod img;
mod our_gl;
mod old;
mod model;

use img::Img;
use our_gl::{Shader, triangle};
// use old::*;

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::{fmt, cmp, f32};
use rand::random;
use image::DecodingResult::{U8, U16};
use image::tga::TGADecoder;
use image::{ImageDecoder, ImageBuffer, Rgb};
use nalgebra::geometry::Point3;
use nalgebra::core::{Vector2, Vector3, Vector4, Matrix4, Matrix2x3};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use cgmath::{SquareMatrix};
use nalgebra::dot;


fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}
fn min(x: f32, y: f32) -> f32 {
    x.min(y)
}

const WHITE: (u8, u8, u8) = (255, 255, 255);
const RED: (u8, u8, u8) = (255, 0, 0);
const GREEN: (u8, u8, u8) = (0, 255, 0);
const BLUE: (u8, u8, u8) = (0, 0, 255);

fn main() {
    render_model_with_shaders();
}

// struct GouraudShaderWithTexture {
//     light_insensity: Vector3<f32>,
//     light_direction: Vector3<f32>,
//     matrix: Matrix4<f32>,
//     texture_matrix: 
// }
// impl Shader for GouraudShaderWithTexture {
//     fn vertex(&mut self, face_vertex: Point3<f32>, face_vertex_normal: Vector3<f32>, vertex_index: usize) -> Point3<f32> {
//         self.light_insensity[vertex_index] = max(0.0, dot(&self.light_direction, &face_vertex_normal));
//         Point3::from_homogeneous(self.matrix * face_vertex.to_homogeneous()).unwrap()
//     }
//     fn fragment(&self, bary_coords: (f32, f32, f32)) -> ((u8, u8, u8), bool) {
//         let bary_vec = Vector3::new(bary_coords.0, bary_coords.1, bary_coords.2);
//         let intensity = dot(&bary_vec, &self.light_insensity);
//         let color = ((255.0 * intensity) as u8, (255.0 * intensity) as u8, (255.0 * intensity) as u8);
//         (color, false)
//     }
// }

struct GouraudShader {
    light_insensity: Vector3<f32>,
    light_direction: Vector3<f32>,
    matrix: Matrix4<f32>,
}
impl Shader for GouraudShader {
    fn vertex(&mut self, face_vertex: Point3<f32>, face_vertex_normal: Vector3<f32>, vertex_index: usize) -> Point3<f32> {
        self.light_insensity[vertex_index] = max(0.0, dot(&self.light_direction, &face_vertex_normal));
        Point3::from_homogeneous(self.matrix * face_vertex.to_homogeneous()).unwrap()
    }
    fn fragment(&self, bary_coords: (f32, f32, f32)) -> ((u8, u8, u8), bool) {
        let bary_vec = Vector3::new(bary_coords.0, bary_coords.1, bary_coords.2);
        let intensity = dot(&bary_vec, &self.light_insensity);
        let color = ((255.0 * intensity) as u8, (255.0 * intensity) as u8, (255.0 * intensity) as u8);
        (color, false)
    }
}

struct GouraudShader6Color {
    light_insensity: Vector3<f32>,
    light_direction: Vector3<f32>,
    matrix: Matrix4<f32>,
}
impl Shader for GouraudShader6Color {
    fn vertex(&mut self, face_vertex: Point3<f32>, face_vertex_normal: Vector3<f32>, vertex_index: usize) -> Point3<f32> {
        self.light_insensity[vertex_index] = max(0.0, dot(&self.light_direction, &face_vertex_normal));
        Point3::from_homogeneous(self.matrix * face_vertex.to_homogeneous()).unwrap()
    }
    fn fragment(&self, bary_coords: (f32, f32, f32)) -> ((u8, u8, u8), bool) {
        let bary_vec = Vector3::new(bary_coords.0, bary_coords.1, bary_coords.2);
        let mut intensity = dot(&bary_vec, &self.light_insensity);
        intensity =
            if intensity > 0.85 { 1.0 }
            else if intensity > 0.60 { 0.80 }
            else if intensity > 0.45 { 0.60 }
            else if intensity > 0.30 { 0.45 }
            else if intensity > 0.15 { 0.30 }
            else { 0.0 };
        let color = ((255.0 * intensity) as u8, (155.0 * intensity) as u8, (0.0 * intensity) as u8);
        (color, false)
    }
}

fn render_model_with_shaders() {
    let (width, height): (f32, f32) = (800.0, 800.0);
    let mut img = Img::new(width as u32, height as u32);

    let mut zbuffer: HashMap<(i32, i32), f32> = HashMap::new();
    let model = model::parse_obj_file("/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj");

    // eye, center, up
    let model_view = lookat(Vector3::new(0.5, 0.25, 1.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
    // Add perspective
    let c = 5.0;
    let project = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0 / c, 1.0);
    // viewport matrix
    let (x, y, d) = (0.0, 0.0, 255.0);
    let viewport = Matrix4::new(
        width / 2.0, 0.0, 0.0, x + width / 2.0,
        0.0, height / 2.0, 0.0, y + height / 2.0,
        0.0, 0.0, d / 2.0, d / 2.0,
        0.0, 0.0, 0.0, 1.0);
    // final matrix
    let m = viewport * project * model_view;
    let m_inv_trans = m4_inverse((project * model_view).transpose());

    let mut gouraud_shader = GouraudShader6Color {
        light_insensity: Vector3::new(0.0, 0.0, 0.0),
        light_direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
        matrix: m,
    };

    for (i, f) in model.faces.iter().enumerate() {
        let screen_pt0 = gouraud_shader.vertex(f.0, model.face_normals[i].0, 0).coords;
        let screen_pt1 = gouraud_shader.vertex(f.1, model.face_normals[i].1, 1).coords;
        let screen_pt2 = gouraud_shader.vertex(f.2, model.face_normals[i].2, 2).coords;

        triangle(screen_pt0, screen_pt1, screen_pt2, &gouraud_shader, &mut img, &mut zbuffer);
    }

    img.flip_vertical();
    img.save("output.png");
}

fn lookat(eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let z = (eye - center).normalize();
    let x = up.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    let m = Matrix4::new(
        x.x, x.y, x.z, 0.0,
        y.x, y.y, y.z, 0.0,
        z.x, z.y, z.z, 0.0,
        0.0, 0.0, 0.0, 1.0);
    let t = Matrix4::new(
        1.0, 0.0, 0.0, -1.0 * center[0],
        0.0, 1.0, 0.0, -1.0 * center[1],
        0.0, 0.0, 1.0, -1.0 * center[2],
        0.0, 0.0, 0.0, 1.0);
    m * t
}

fn m4_inverse(m: Matrix4<f32>) -> Matrix4<f32> {
    let temp = m.iter().map(|x| *x).collect::<Vec<f32>>();
    let mut m2 = cgmath::Matrix4::new(
        temp[0], temp[1], temp[2], temp[3],
        temp[4], temp[5], temp[6], temp[7],
        temp[8], temp[9], temp[10], temp[11],
        temp[12], temp[13], temp[14], temp[15]);
    m2 = m2.invert().unwrap();

    Matrix4::new(
        m2.x.x, m2.y.x, m2.z.x, m2.w.z,
        m2.x.y, m2.y.y, m2.z.y, m2.w.y,
        m2.x.z, m2.y.z, m2.z.z, m2.w.z,
        m2.x.w, m2.y.w, m2.z.w, m2.w.w)
}

fn barycentric(p: Vector2<f32>, t: (Vector2<f32>, Vector2<f32>, Vector2<f32>)) -> (f32, f32, f32) {
    let b1 = ((t.1.y - t.2.y) * (p.x - t.2.x) + (t.2.x - t.1.x) * (p.y - t.2.y)) /
        ((t.1.y - t.2.y) * (t.0.x - t.2.x) + (t.2.x - t.1.x) * (t.0.y - t.2.y));

    let b2 = ((t.2.y - t.0.y) * (p.x - t.2.x) + (t.0.x - t.2.x) * (p.y - t.2.y)) /
        ((t.1.y - t.2.y) * (t.0.x - t.2.x) + (t.2.x - t.1.x) * (t.0.y - t.2.y));

    let b3 = 1.0 - b1 - b2;
    (b1, b2, b3)
}

fn read_texture(path: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut decoder = TGADecoder::new(File::open(path).unwrap());
    let (width, height) = decoder.dimensions().unwrap();
    let color_type = decoder.colortype().unwrap();
    println!("color_type is {:?}, width is {}, height is {}", color_type, width, height);
    let texture = match decoder.read_image().unwrap() {
        U8(vec) => {
            ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, vec).unwrap()
        },
        U16(_) => {
            // TODO: should probably fix this?
            let v: Vec<u8> = Vec::new();
            ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, v).unwrap()
        },
    };
    texture
}
