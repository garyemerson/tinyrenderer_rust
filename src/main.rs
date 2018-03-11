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

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::{fmt, cmp, f32};
use rand::random;
use image::DecodingResult::{U8, U16};
use image::tga::TGADecoder;
use image::{ImageDecoder, ImageBuffer, Rgb};
use nalgebra::geometry::Point3;
use nalgebra::core::{Vector3, Vector2, Matrix4};
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
    run_model_with_zbuffer_and_perspective();
}

fn run_model_with_zbuffer_and_perspective() {
    let (width, height): (f32, f32) = (800.0, 800.0);
    let mut img = Img::new(width as u32, height as u32);

    model_with_zbuffer_and_perspective(
        "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj",
        "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head_diffuse.tga",
        &mut img,
        width,
        height);

    img.flip_vertical();
    img.save("output.png");
}

fn model_with_zbuffer_and_perspective(model_path: &str, texture_path: &str, img: &mut Img, width: f32, height: f32) {
    // let mut zbuffer = vec![vec![-f32::MAX; (height + 20.0) as usize]; (width + 20.0) as usize];
    let mut zbuffer: HashMap<(i32, i32), f32> = HashMap::new();
    let model = model::parse_obj_file(model_path);
    println!("{} texture vertices", model.face_texture_vertices.len());
    let texture = read_texture(texture_path);

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

    for (i, f) in model.faces.iter().enumerate() {
        // println!("processing face {}: {:?}", i, f);

        let p0 = Point3::from_homogeneous(m * f.0.to_homogeneous()).unwrap();
        let p1 = Point3::from_homogeneous(m * f.1.to_homogeneous()).unwrap();
        let p2 = Point3::from_homogeneous(m * f.2.to_homogeneous()).unwrap();

        let vn0 = model.face_normals[i].0;
        let vn1 = model.face_normals[i].1;
        let vn2 = model.face_normals[i].2;

        // println!("m_inv_trans is {}", m_inv_trans);
        // println!("vn0 is {}, m_inv_trans * vn0 is {}", vn0, m_inv_trans * vn0.to_homogeneous());
        // println!("vn1 is {}, m_inv_trans * vn1 is {}", vn1, m_inv_trans * vn1.to_homogeneous());
        // println!("vn2 is {}, m_inv_trans * vn2 is {}", vn2, m_inv_trans * vn2.to_homogeneous());
        let vn0 = Vector3::from_homogeneous(m_inv_trans * vn0.to_homogeneous()).unwrap().normalize();
        let vn1 = Vector3::from_homogeneous(m_inv_trans * vn1.to_homogeneous()).unwrap().normalize();
        let vn2 = Vector3::from_homogeneous(m_inv_trans * vn2.to_homogeneous()).unwrap().normalize();

        let u = p1 - p0;
        let v = p2 - p0;
        let normal = u.cross(&v).normalize();

        // let light_dir = Vector3::new(0.0, 1.0, 0.75).normalize();
        let light_dir = Vector3::new(1.0, 0.0, 2.0).normalize();
        let light_insensity = dot(&light_dir, &normal);
        // println!("light_insensity is {}", light_insensity);
        if light_insensity > 0.0 {
            // let (vt0, vt1, vt2) = ((f.1).0 - 1, (f.1).1 - 1, (f.1).2 - 1);
            // println!("vt0 is {}, vt1 is {}, vt2 is {}", vt0, vt1, vt2);

            // let ptx0 = Vector2::new(model.texture_vertices[vt0].0 * 1024.0, model.texture_vertices[vt0].1 * 1024.0);
            // let ptx1 = Vector2::new(model.texture_vertices[vt1].0 * 1024.0, model.texture_vertices[vt1].1 * 1024.0);
            // let ptx2 = Vector2::new(model.texture_vertices[vt2].0 * 1024.0, model.texture_vertices[vt2].1 * 1024.0);

            // light direction is (0, 0, 1)
            let light_intensity0 = dot(&light_dir, &vn0);
            let light_intensity1 = dot(&light_dir, &vn1);
            let light_intensity2 = dot(&light_dir, &vn2);

            // println!("point coords: {:?}, {:?}, {:?}", p0.coords, p1.coords, p2.coords);
            triangle_with_zbuff_and_texture(
                p0.coords,
                p1.coords,
                p2.coords,
                light_intensity0,
                light_intensity1,
                light_intensity2,
                img,
                &texture,
                &mut zbuffer);
        }
    }
}

fn triangle_with_zbuff_and_texture(
    p0: Vector3<f32>,
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    light_intensity0: f32,
    light_intensity1: f32,
    light_intensity2: f32,
    img: &mut Img,
    texture: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    zbuffer: &mut HashMap<(i32, i32), f32>)
{
    let _ = texture;
    let bb_up_right = Vector2::<i32>::new(max(p0.x, max(p1.x, p2.x)) as i32, max(0.0, min(p0.y, min(p1.y, p2.y))) as i32);
    let bb_lower_left = Vector2::<i32>::new(max(0.0, min(p0.x, min(p1.x, p2.x))) as i32, max(p0.y, max(p1.y, p2.y)) as i32);

    for x in bb_lower_left.x..(bb_up_right.x + 1) {
        for y in bb_up_right.y..(bb_lower_left.y + 1) {
            let bary = barycentric(Vector2::new(x as f32, y as f32), (p0.remove_row(2), p1.remove_row(2), p2.remove_row(2)));

            if bary.0 >= 0.0 && bary.1 >= 0.0 && bary.2 >= 0.0 {
                // ???: How is this the z coord
                let mut z = p0.z as f32 * bary.0
                    + p1.z as f32 * bary.1
                    + p2.z as f32 * bary.2;

                // // coordinate in the texture
                // let tcoord = bary_to_cart(
                //     (ptx0, ptx1, ptx2),
                //     bary);
                // // texture pixel to use
                // let tpx = texture.get_pixel(tcoord.x as u32, (1024.0 - tcoord.y) as u32);
                // let color = (
                //     (tpx[0] as f32 * light_insensity) as u8,
                //     (tpx[1] as f32 * light_insensity) as u8,
                //     (tpx[2] as f32 * light_insensity) as u8);

                let mut light_insensity = light_intensity0 * bary.0 + light_intensity1 * bary.1 + light_intensity2 * bary.2;
                // Not sure how some of li params are negative but there are so protect against
                // negative light_insensity
                if light_insensity < 0.0 {
                    light_insensity = 0.0;
                }
                // light_insensity = (f32::sin((light_insensity - 1.0) * f32::consts::PI/2.0) + 1.0) / 2.0;
                let color = ((light_insensity * 255.0) as u8, (light_insensity * 255.0) as u8, (light_insensity * 255.0) as u8);

                match zbuffer.entry((x as i32, y as i32)) {
                    Occupied(mut e) => {
                        let mut val = e.get_mut();
                        if *val < z {
                            *val = z;
                            img.set(x as u32, y as u32, color);
                        }
                    },
                    Vacant(e) => {
                        e.insert(z);
                        img.set(x as u32, y as u32, color);
                    }
                }
            }
        }
    }
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
