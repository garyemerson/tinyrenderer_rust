extern crate regex;
extern crate image;
extern crate rand;

mod img;
use img::Img;

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;
use std::mem;
use std::fmt;
use std::cmp::{min, max};
use rand::random;
use std::f32;
use image::DecodingResult::{U8, U16};
use image::tga::TGADecoder;
use image::{ImageDecoder, ImageBuffer, Rgb};
use std::path::Path;

#[derive(Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}
impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
struct Pt3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Pt3 {
    fn pt2(&self) -> Pt {
        Pt { x: self.x as i32, y: self.y as i32 }
    }
}
impl fmt::Display for Pt3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

const WHITE: (u8, u8, u8) = (255, 255, 255);
const RED: (u8, u8, u8) = (255, 0, 0);
const GREEN: (u8, u8, u8) = (0, 255, 0);
const BLUE: (u8, u8, u8) = (0, 0, 255);

fn main() {
    let (width, height): (f32, f32) = (800.0, 800.0);
    let mut img = Img::new(width as u32, height as u32);

    // line_benchmark(&mut img);
    // draw_wireframe("/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj", &mut img, width, height);
    // triangle_exercises(&mut img);
    // rand_triangle_model(
    //     "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj",
    //     &mut img,
    //     width,
    //     height);
    // lit_triangle_model(
    //     "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj",
    //     &mut img,
    //     width,
    //     height);

    // // TODO: Fix this weird pixel crap
    // let (width, height): (f32, f32) = (800.0, 800.0);
    // let mut img = Img::new(width as u32, height as u32);
    // let t = (Pt{x: 10, y: 20}, Pt{x: 40, y: 20}, Pt{x: 30, y: 40});
    // let p = Pt{x: 10, y: 10};
    // draw_triangle3(t.0, t.1, t.2, &mut img, RED);

    // let t = (Pt{x: 100, y: 200}, Pt{x: 400, y: 200}, Pt{x: 300, y: 400});
    // let p = Pt{x: 300, y: 300};
    // let c = bary_to_cart(t, (0.33, 0.33, 0.33));
    // println!("{:?}", barycentric(p, t));
    // draw_triangle3(t.0, t.1, t.2, &mut img, RED);
    // img.set(p.x as u32, p.y as u32, GREEN);
    // img.set(c.x as u32, c.x as u32, BLUE);

    // // scene "2d mesh"
    // line4(20, 34, 744, 400, &mut img, RED);
    // line4(120, 434, 444, 400, &mut img, GREEN);
    // line4(330, 463, 594, 200, &mut img, BLUE);
    // // screen line
    // line4(10, 10, 790, 10, &mut img, WHITE);

    // let mut ybuffer: Vec<i32> = vec![i32::min_value(); width as usize];
    // rasterize(Pt { x: 20, y: 34 },   Pt { x: 744, y: 400 }, &mut img, RED,   &mut ybuffer);
    // rasterize(Pt { x: 120, y: 434 }, Pt { x: 444, y: 400 }, &mut img, GREEN, &mut ybuffer);
    // rasterize(Pt { x: 330, y: 463 }, Pt { x: 594, y: 200 }, &mut img, BLUE,  &mut ybuffer);

    // model_with_zbuffer(
    //     "/Users/Garrett/Downloads/snapchat-dancing-hotdog-meme-whole-hotdog.stl.obj",
    //     // "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj",
    //     &mut img,
    //     width,
    //     height);

    // model_with_zbuffer_and_texture(
    //     "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj",
    //     "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head_diffuse.tga",
    //     &mut img,
    //     width,
    //     height);

    model_with_zbuffer_texture_perspective(
        "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj",
        "/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head_diffuse.tga",
        &mut img,
        width,
        height);

    img.flip_vertical();
    img.save("output.png");
}

fn world_to_screen(p: Pt3, width: f32, height: f32) -> Pt3 {
    Pt3 {
        x: ((p.x as f32 + 1.0) * width / 2.0 + 0.5),
        y: ((p.y as f32 + 1.0) * height / 2.0 + 0.5),
        z: p.z,
    }
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

fn model_with_zbuffer_texture_perspective(model_path: &str, texture_path: &str, img: &mut Img, width: f32, height: f32) {
    let mut zbuffer = vec![vec![-f32::MAX; (height + 1.0) as usize]; (width + 1.0) as usize];
    let (vertices, faces, texture_vertices) = parse_obj_file(model_path);
    println!("{} texture vertices", texture_vertices.len());
    let texture = read_texture(texture_path);

    for f in faces {
        let p0 = Pt3 { 
            x: vertices[((f.0).0 - 1)].0,
            y: vertices[((f.0).0 - 1)].1,
            z: vertices[((f.0).0 - 1)].2,
        };
        let p1 = Pt3 { 
            x: vertices[((f.0).1 - 1)].0,
            y: vertices[((f.0).1 - 1)].1,
            z: vertices[((f.0).1 - 1)].2,
        };
        let p2 = Pt3 { 
            x: vertices[((f.0).2 - 1)].0,
            y: vertices[((f.0).2 - 1)].1,
            z: vertices[((f.0).2 - 1)].2,
        };

        let u = (p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
        let v = (p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
        let mut norm = (
            (u.1 * v.2 - u.2 * v.1),
            (u.2 * v.0 - u.0 * v.2),
            (u.0 * v.1 - u.1 * v.0));
        let mag = (norm.0.powi(2) + norm.1.powi(2) + norm.2.powi(2)).sqrt();
        norm.0 = norm.0 / mag;
        norm.1 = norm.1 / mag;
        norm.2 = norm.2 / mag;

        // Add perspective
        let c = 5.0;
        let p0p = Pt3 { x: p0.x / (1.0 - p0.z / c), y: p0.y / (1.0 - p0.z / c), z: p0.z / (1.0 - p0.z / c)};
        let p1p = Pt3 { x: p1.x / (1.0 - p1.z / c), y: p1.y / (1.0 - p1.z / c), z: p1.z / (1.0 - p1.z / c)};
        let p2p = Pt3 { x: p2.x / (1.0 - p2.z / c), y: p2.y / (1.0 - p2.z / c), z: p2.z / (1.0 - p2.z / c)};

        let p0s = world_to_screen(p0p, width, height);
        let p1s = world_to_screen(p1p, width, height);
        let p2s = world_to_screen(p2p, width, height);

        // light direction is (0, 0, 1)
        let light_insensity = 1.0 * norm.2;
        // println!("light_insensity is {}", light_insensity);
        if light_insensity > 0.0 {
            let (vt0, vt1, vt2) = ((f.1).0 - 1, (f.1).1 - 1, (f.1).2 - 1);
            // println!("vt0 is {}, vt1 is {}, vt2 is {}", vt0, vt1, vt2);
            let (ptx0, ptx1, ptx2) = (
                Pt { x: (texture_vertices[vt0].0 * 1024.0) as i32, y: (texture_vertices[vt0].1 * 1024.0) as i32 },
                Pt { x: (texture_vertices[vt1].0 * 1024.0) as i32, y: (texture_vertices[vt1].1 * 1024.0) as i32 },
                Pt { x: (texture_vertices[vt2].0 * 1024.0) as i32, y: (texture_vertices[vt2].1 * 1024.0) as i32 });
            triangle_with_zbuff_and_texture(
                p0s,
                p1s,
                p2s,
                ptx0,
                ptx1,
                ptx2,
                img,
                light_insensity,
                &texture,
                &mut zbuffer);
        }
    }
}

fn model_with_zbuffer_and_texture(model_path: &str, texture_path: &str, img: &mut Img, width: f32, height: f32) {
    let mut zbuffer = vec![vec![-f32::MAX; (height + 1.0) as usize]; (width + 1.0) as usize];
    let (vertices, faces, texture_vertices) = parse_obj_file(model_path);
    println!("{} texture vertices", texture_vertices.len());
    let texture = read_texture(texture_path);

    for f in faces {
        let p0 = Pt3 { 
            x: vertices[((f.0).0 - 1)].0,
            y: vertices[((f.0).0 - 1)].1,
            z: vertices[((f.0).0 - 1)].2,
        };
        let p1 = Pt3 { 
            x: vertices[((f.0).1 - 1)].0,
            y: vertices[((f.0).1 - 1)].1,
            z: vertices[((f.0).1 - 1)].2,
        };
        let p2 = Pt3 { 
            x: vertices[((f.0).2 - 1)].0,
            y: vertices[((f.0).2 - 1)].1,
            z: vertices[((f.0).2 - 1)].2,
        };

        let u = (p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
        let v = (p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
        let mut norm = (
            (u.1 * v.2 - u.2 * v.1),
            (u.2 * v.0 - u.0 * v.2),
            (u.0 * v.1 - u.1 * v.0));
        let mag = (norm.0.powi(2) + norm.1.powi(2) + norm.2.powi(2)).sqrt();
        norm.0 = norm.0 / mag;
        norm.1 = norm.1 / mag;
        norm.2 = norm.2 / mag;

        let p0s = world_to_screen(p0, width, height);
        let p1s = world_to_screen(p1, width, height);
        let p2s = world_to_screen(p2, width, height);

        // light direction is (0, 0, 1)
        let light_insensity = 1.0 * norm.2;
        // println!("light_insensity is {}", light_insensity);
        if light_insensity > 0.0 {
            let (vt0, vt1, vt2) = ((f.1).0 - 1, (f.1).1 - 1, (f.1).2 - 1);
            // println!("vt0 is {}, vt1 is {}, vt2 is {}", vt0, vt1, vt2);
            let (ptx0, ptx1, ptx2) = (
                Pt { x: (texture_vertices[vt0].0 * 1024.0) as i32, y: (texture_vertices[vt0].1 * 1024.0) as i32 },
                Pt { x: (texture_vertices[vt1].0 * 1024.0) as i32, y: (texture_vertices[vt1].1 * 1024.0) as i32 },
                Pt { x: (texture_vertices[vt2].0 * 1024.0) as i32, y: (texture_vertices[vt2].1 * 1024.0) as i32 });
            triangle_with_zbuff_and_texture(
                p0s,
                p1s,
                p2s,
                ptx0,
                ptx1,
                ptx2,
                img,
                light_insensity,
                &texture,
                &mut zbuffer);
        }
    }
}

fn model_with_zbuffer(model_path: &str, img: &mut Img, width: f32, height: f32) {
    let mut zbuffer = vec![vec![-f32::MAX; (height + 1.0) as usize]; (width + 1.0) as usize];
    let (vertices, faces, texture_vertices) = parse_obj_file(model_path);
    println!("{} vertices, {} faces", vertices.len(), faces.len());

    for f in faces {
        let p0 = Pt3 { 
            x: vertices[((f.0).0 - 1)].0,
            y: vertices[((f.0).0 - 1)].1,
            z: vertices[((f.0).0 - 1)].2,
        };
        let p1 = Pt3 { 
            x: vertices[((f.0).1 - 1)].0,
            y: vertices[((f.0).1 - 1)].1,
            z: vertices[((f.0).1 - 1)].2,
        };
        let p2 = Pt3 { 
            x: vertices[((f.0).2 - 1)].0,
            y: vertices[((f.0).2 - 1)].1,
            z: vertices[((f.0).2 - 1)].2,
        };

        let u = (p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
        let v = (p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
        let mut norm = (
            (u.1 * v.2 - u.2 * v.1),
            (u.2 * v.0 - u.0 * v.2),
            (u.0 * v.1 - u.1 * v.0));
        let mag = (norm.0.powi(2) + norm.1.powi(2) + norm.2.powi(2)).sqrt();
        norm.0 = norm.0 / mag;
        norm.1 = norm.1 / mag;
        norm.2 = norm.2 / mag;

        let p0s = world_to_screen(p0, width, height);
        let p1s = world_to_screen(p1, width, height);
        let p2s = world_to_screen(p2, width, height);

        // light direction is (0, 0, 1)
        let light_insensity = 1.0 * norm.2;
        // println!("light_insensity is {}", light_insensity);
        if light_insensity > 0.0 {
            // triangle_with_zbuff(
            //     p0s,
            //     p1s,
            //     p2s,
            //     img,
            //     ((light_insensity * 255.0) as u8, (light_insensity * 255.0) as u8, (light_insensity * 255.0) as u8),
            //     &mut zbuffer);

            triangle_with_zbuff(
                p0s,
                p1s,
                p2s,
                img,
                ((255.0 * light_insensity) as u8, (255.0 * light_insensity) as u8, (255.0 * light_insensity) as u8),
                &mut zbuffer);
        }
    }
}

fn triangle_with_zbuff_and_texture(
    p0: Pt3,
    p1: Pt3,
    p2: Pt3,
    ptx0: Pt,
    ptx1: Pt,
    ptx2: Pt,
    img: &mut Img,
    light_insensity: f32,
    texture: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    zbuffer: &mut Vec<Vec<f32>>)
{
    let bb_up_right = Pt { x: max(p0.x as i32, max(p1.x as i32, p2.x as i32)), y: max(0, min(p0.y as i32, min(p1.y as i32, p2.y as i32))) };
    let bb_lower_left = Pt { x: max(0, min(p0.x as i32, min(p1.x as i32, p2.x as i32))), y: max(p0.y as i32, max(p1.y as i32, p2.y as i32)) };

    for x in bb_lower_left.x..(bb_up_right.x + 1) {
        for y in bb_up_right.y..(bb_lower_left.y + 1) {
            let bary = barycentric(Pt { x, y }, (p0.pt2(), p1.pt2(), p2.pt2()));

            if bary.0 >= 0.0 && bary.1 >= 0.0 && bary.2 >= 0.0 {
                // ???: How is this the z coord
                let mut z = p0.z as f32 * bary.0
                    + p1.z as f32 * bary.1
                    + p2.z as f32 * bary.2;

                // coordinate in the texture
                let tcoord = bary_to_cart(
                    (ptx0, ptx1, ptx2),
                    bary);
                // texture pixel to use
                let tpx = texture.get_pixel(tcoord.x as u32, (1024 - tcoord.y) as u32);
                let color = (
                    (tpx[0] as f32 * light_insensity) as u8,
                    (tpx[1] as f32 * light_insensity) as u8,
                    (tpx[2] as f32 * light_insensity) as u8);

                if zbuffer[x as usize][y as usize] < z {
                    zbuffer[x as usize][y as usize] = z;
                    img.set(x as u32, y as u32, color);
                }
            }
        }
    }
}

fn triangle_with_zbuff(
    p0: Pt3,
    p1: Pt3,
    p2: Pt3,
    img: &mut Img,
    color: (u8, u8, u8),
    zbuffer: &mut Vec<Vec<f32>>)
{
    let bb_up_right = Pt { x: max(p0.x as i32, max(p1.x as i32, p2.x as i32)), y: max(0, min(p0.y as i32, min(p1.y as i32, p2.y as i32))) };
    let bb_lower_left = Pt { x: max(0, min(p0.x as i32, min(p1.x as i32, p2.x as i32))), y: max(p0.y as i32, max(p1.y as i32, p2.y as i32)) };

    for x in bb_lower_left.x..(bb_up_right.x + 1) {
        for y in bb_up_right.y..(bb_lower_left.y + 1) {
            let bary = barycentric(Pt { x, y }, (p0.pt2(), p1.pt2(), p2.pt2()));

            if bary.0 >= 0.0 && bary.1 >= 0.0 && bary.2 >= 0.0 {
                // ???: How is this the z coord
                let mut z = p0.z as f32 * bary.0
                    + p1.z as f32 * bary.1
                    + p2.z as f32 * bary.2;
                if zbuffer[x as usize][y as usize] < z {
                    zbuffer[x as usize][y as usize] = z;
                    img.set(x as u32, y as u32, color);
                }
            }
        }
    }
}

fn rasterize(mut p0: Pt, mut p1: Pt, img: &mut Img, color: (u8, u8, u8), ybuffer: &mut Vec<i32>) {
    if p0.x > p1.x { mem::swap(&mut p0, &mut p1); }

    for x in p0.x..(p1.x + 1) {
        let t: f32 = (x - p0.x) as f32 / (p1.x - p0.x) as f32;
        let y = p0.y + (t * (p1.y - p0.y) as f32) as i32;

        if ybuffer[x as usize] < y {
            ybuffer[x as usize] = y;
            img.set(x as u32, 0, color);
        }
    }
}

fn lit_triangle_model(path: &str, img: &mut Img, width: f32, height: f32) {
    let (vertices, faces, _) = parse_obj_file(path);
    println!("{} vertices, {} faces", vertices.len(), faces.len());

    for f in faces {
        let p0 = Pt { 
            x: ((vertices[((f.0).0 - 1)].0 + 1.0) * width / 2.0).floor() as i32,
            y: ((vertices[((f.0).0 - 1)].1 + 1.0) * height / 2.0).floor() as i32
        };
        let p1 = Pt { 
            x: ((vertices[((f.0).1 - 1)].0 + 1.0) * width / 2.0).floor() as i32,
            y: ((vertices[((f.0).1 - 1)].1 + 1.0) * height / 2.0).floor() as i32
        };
        let p2 = Pt { 
            x: ((vertices[((f.0).2 - 1)].0 + 1.0) * width / 2.0).floor() as i32,
            y: ((vertices[((f.0).2 - 1)].1 + 1.0) * height / 2.0).floor() as i32
        };

        let u = (
            vertices[((f.0).1 - 1)].0 - vertices[((f.0).0 - 1)].0,
            vertices[((f.0).1 - 1)].1 - vertices[((f.0).0 - 1)].1,
            vertices[((f.0).1 - 1)].2 - vertices[((f.0).0 - 1)].2);
        let v = (
            vertices[((f.0).2 - 1)].0 - vertices[((f.0).0 - 1)].0,
            vertices[((f.0).2 - 1)].1 - vertices[((f.0).0 - 1)].1,
            vertices[((f.0).2 - 1)].2 - vertices[((f.0).0 - 1)].2);
        let mut norm = (
            (u.1 * v.2 - u.2 * v.1),
            (u.2 * v.0 - u.0 * v.2),
            (u.0 * v.1 - u.1 * v.0));
        let mag = (norm.0.powi(2) + norm.1.powi(2) + norm.2.powi(2)).sqrt();
        norm.0 = norm.0 / mag;
        norm.1 = norm.1 / mag;
        norm.2 = norm.2 / mag;
        // light direction is (0, 0, 1)
        let light_insensity = 1.0 * norm.2;
        // println!("light_insensity is {}", light_insensity);
        if light_insensity > 0.0 {
            draw_triangle3(
                p0,
                p1,
                p2,
                img,
                ((light_insensity * 255.0) as u8, (light_insensity * 255.0) as u8, (light_insensity * 255.0) as u8));
        }
    }
}

fn rand_triangle_model(path: &str, img: &mut Img, width: f32, height: f32) {
    let (vertices, faces, _) = parse_obj_file(path);
    println!("{} vertices, {} faces", vertices.len(), faces.len());

    for f in faces {
        let p0 = Pt { 
            x: ((vertices[((f.0).0 - 1)].0 + 1.0) * width / 2.0).floor() as i32,
            y: ((vertices[((f.0).0 - 1)].1 + 1.0) * height / 2.0).floor() as i32
        };
        let p1 = Pt { 
            x: ((vertices[((f.0).1 - 1)].0 + 1.0) * width / 2.0).floor() as i32,
            y: ((vertices[((f.0).1 - 1)].1 + 1.0) * height / 2.0).floor() as i32
        };
        let p2 = Pt { 
            x: ((vertices[((f.0).2 - 1)].0 + 1.0) * width / 2.0).floor() as i32,
            y: ((vertices[((f.0).2 - 1)].1 + 1.0) * height / 2.0).floor() as i32
        };
        draw_triangle3(p0, p1, p2, img, (random::<u8>(), random::<u8>(), random::<u8>()));
    }
}

fn triangle_exercises(img: &mut Img) {
    // let t0 = [ Pt { x: 10, y: 70 }, Pt { x: 50, y: 160 }, Pt { x: 70, y: 80 } ]; 
    // let t1 = [ Pt { x: 180, y: 50 }, Pt { x: 150, y: 1 }, Pt { x: 70, y: 180 } ];
    // let t2 = [ Pt { x: 180, y: 150 }, Pt { x: 120, y: 160 }, Pt { x: 130, y: 180 } ];
    // draw_triangle3(t0[0], t0[1], t0[2], &mut img, RED); 
    // draw_triangle3(t1[0], t1[1], t1[2], &mut img, WHITE); 
    // draw_triangle3(t2[0], t2[1], t2[2], &mut img, GREEN);

    // let t = [ Pt { x: 10, y: 10 }, Pt { x: 100, y:  30 }, Pt { x: 190, y:  160 } ];
    let t = [ Pt { x: 10, y: 10 }, Pt { x: 100, y:  30 }, Pt { x: 100, y:  160 } ];
    draw_triangle3(t[0], t[1], t[2], img, RED); 
}

fn bary_to_cart(t: (Pt, Pt, Pt), b: (f32, f32, f32)) -> Pt {
    let p = (
        b.0 * t.0.x as f32 + b.1 * t.1.x as f32 + b.2 * t.2.x as f32,
        b.0 * t.0.y as f32 + b.1 * t.1.y as f32 + b.2 * t.2.y as f32);
    Pt { x: p.0 as i32, y: p.1 as i32 }
}

fn barycentric(p: Pt, t: (Pt, Pt, Pt)) -> (f32, f32, f32) {
    let b1 = ((t.1.y - t.2.y) * (p.x - t.2.x) + (t.2.x - t.1.x) * (p.y - t.2.y)) as f32 /
        ((t.1.y - t.2.y) * (t.0.x - t.2.x) + (t.2.x - t.1.x) * (t.0.y - t.2.y)) as f32;

    let b2 = ((t.2.y - t.0.y) * (p.x - t.2.x) + (t.0.x - t.2.x) * (p.y - t.2.y)) as f32 /
        ((t.1.y - t.2.y) * (t.0.x - t.2.x) + (t.2.x - t.1.x) * (t.0.y - t.2.y)) as f32;

    let b3 = 1.0 - b1 - b2;
    (b1, b2, b3)
}
fn inside_triangle(p: Pt, t: (Pt, Pt, Pt)) -> bool {
    let b = barycentric(p, t);
    b.0 >= 0.0 && b.1 >= 0.0 && b.2 >= 0.0
}
fn draw_triangle3(p0: Pt, p1: Pt, p2: Pt, img: &mut Img, color: (u8, u8, u8)) {
    // bounding box points
    let bb_up_right = Pt { x: max(p0.x, max(p1.x, p2.x)), y: min(p0.y, min(p1.y, p2.y)) };
    let bb_lower_left = Pt { x: min(p0.x, min(p1.x, p2.x)), y: max(p0.y, max(p1.y, p2.y)) };

    for x in bb_lower_left.x..(bb_up_right.x + 1) {
        for y in bb_up_right.y..(bb_lower_left.y + 1) {
            if inside_triangle(Pt { x, y }, (p0, p1, p2)) {
                img.set(x as u32, y as u32, color);
            }
        }
    }
}

fn draw_triangle2(mut p0: Pt, mut p1: Pt, mut p2: Pt, img: &mut Img) {
    if p0.y > p1.y { mem::swap(&mut p0, &mut p1); }
    if p1.y > p2.y { mem::swap(&mut p1, &mut p2); }
    if p0.y > p1.y { mem::swap(&mut p1, &mut p2); }

    // println!("drawing triangle {} {} {}", p0, p1, p2);

    let total_height = p2.y - p0.y;
    for y in p0.y..(p1.y + 1) {
        let segment_height = p1.y - p0.y + 1;
        let alpha = (y - p0.y) as f32 / total_height as f32;
        let beta = (y - p0.y) as f32 / segment_height as f32;
        let mut ax = p0.x + ((p2.x - p0.x) as f32 * alpha) as i32;
        let mut bx = p0.x + ((p1.x - p0.x) as f32 * beta) as i32;

        if ax > bx { mem::swap(&mut ax, &mut bx); }
        // println!("sweeping y horizontal {} from x {} to {}", y, ax, bx);
        for x in ax..(bx + 1) {
            // println!("setting px ({}, {})", x, y);
            img.set(x as u32, y as u32, WHITE);
        }
    }
    // println!("upper half");
    for y in p1.y..(p2.y + 1) {
        let segment_height = p2.y - p1.y + 1;
        let alpha = (y - p0.y) as f32 / total_height as f32;
        let beta = (y - p1.y) as f32 / segment_height as f32;
        let mut ax = p0.x + ((p2.x - p0.x) as f32 * alpha) as i32;
        let mut bx = p1.x + ((p2.x - p1.x) as f32 * beta) as i32;

        if ax > bx { mem::swap(&mut ax, &mut bx); }
        for x in ax..(bx + 1) {
            // println!("setting px ({}, {})", x, y);
            img.set(x as u32, y as u32, WHITE);
        }
    }
}

fn draw_triangle(p0: Pt, p1: Pt, p2: Pt, img: &mut Img, color: (u8, u8, u8)) { 
    line4(p0.x as u32, p0.y as u32, p1.x as u32, p1.y as u32, img, color); 
    line4(p1.x as u32, p1.y as u32, p2.x as u32, p2.y as u32, img, color); 
    line4(p2.x as u32, p2.y as u32, p0.x as u32, p0.y as u32, img, color); 
}

fn line_benchmark(img: &mut Img) {
    for _ in 0..1000000 {
        line4(50, 50, 50, 60, img, WHITE);
        line4(50, 50, 50, 40, img, WHITE);
        line4(50, 50, 60, 60, img, WHITE);
        line4(50, 50, 60, 50, img, WHITE);
        line4(50, 50, 40, 50, img, WHITE);
        line4(50, 50, 60, 40, img, WHITE);
        line4(50, 50, 40, 60, img, WHITE);
        line4(50, 50, 40, 40, img, WHITE);
    }
}

fn draw_wireframe(path: &str, img: &mut Img, width: f32, height: f32) {
    let (vertices, faces, _) = parse_obj_file(path);
    println!("{} vertices, {} faces", vertices.len(), faces.len());
    for f in faces {
        line4(
            ((vertices[((f.0).0 - 1)].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[((f.0).0 - 1)].1 + 1.0) * height / 2.0).floor() as u32,
            ((vertices[((f.0).1 - 1)].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[((f.0).1 - 1)].1 + 1.0) * height / 2.0).floor() as u32,
            img, WHITE);
        line4(
            ((vertices[((f.0).1 - 1)].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[((f.0).1 - 1)].1 + 1.0) * height / 2.0).floor() as u32,
            ((vertices[((f.0).2 - 1)].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[((f.0).2 - 1)].1 + 1.0) * height / 2.0).floor() as u32,
            img, WHITE);
        line4(
            ((vertices[((f.0).2 - 1)].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[((f.0).2 - 1)].1 + 1.0) * height / 2.0).floor() as u32,
            ((vertices[((f.0).0 - 1)].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[((f.0).0 - 1)].1 + 1.0) * height / 2.0).floor() as u32,
            img, WHITE);
    }
}

fn line4(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    let mut steep = false; 
    if ((x0 as i32) - (x1 as i32)).abs() < ((y0 as i32) - (y1 as i32)).abs() { // if the line is steep, we transpose the image 
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    } 
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    // println!("drawing line ({}, {}) ({}, {})", x0, y0, x1, y1);

    let dx = ((x0 as i32) - (x1 as i32)).abs();
    let dy = ((y0 as i32) - (y1 as i32)).abs();
    // make this signed bc of the corner case where the last px is on the 0 boundary and the y
    // increment logic will subtract 1 and underflow if unsigned
    let mut y: i32 = y0 as i32;
    let mut error2: i32 = 0;

    for x in x0..(x1 + 1) {
        if steep {img.set(y as u32, x as u32, color); } else {img.set(x as u32, y as u32, color);}

        if 2 * (error2 + dy) < dx {
            error2 = error2 + dy;
        } else {
            y = if y1 > y0 { y + 1 } else { y - 1 };
            error2 = error2 + dy - dx; 
        }
    }
}

fn line3(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    let mut steep = false; 
    if ((x0 as i32) - (x1 as i32)).abs() < ((y0 as i32) - (y1 as i32)).abs() { // if the line is steep, we transpose the image 
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    } 
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = ((x0 as i32) - (x1 as i32)).abs();
    let dy = ((y0 as i32) - (y1 as i32)).abs();
    let derror: f64 = (dy as f64) / (dx as f64);
    let mut error = 0.0;
    let mut y = y0;

    for x in x0..(x1 + 1) {
        if steep {
            img.set(y, x, color);
        } else {
            img.set(x, y, color);
        }

        error += derror;
        if error > 0.5 {
            y = if y1 > y0 { y + 1 } else { y - 1 };
            error -= 1.0;
        }
    }
}

fn line2(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    let mut steep = false; 
    if ((x0 as i32) - (x1 as i32)).abs() < ((y0 as i32) - (y1 as i32)).abs() { // if the line is steep, we transpose the image 
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    } 
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    for x in x0..(x1 + 1) {
        let t = ((x - x0) as f64) / ((x1 - x0) as f64);
        let y = (y0 as f64) * (1f64 - t) + (y1 as f64) * t;

        if steep {
            img.set(y.round() as u32, x, color);
        } else {
            img.set(x, y.round() as u32, color);
        }
    }
}

fn line(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    for step in 0..100 {
        let t = (step as f64) / 100f64;
        let x = (x0 as f64) * (1f64 - t) + (x1 as f64) * t;
        let y = (y0 as f64) * (1f64 - t) + (y1 as f64) * t;
        img.set(x.round() as u32, y.round() as u32, color);
    }
}

// Returns (vertices, (face vertices, face texture vertices), texture vertices)
fn parse_obj_file(path: &str) -> (
    Vec<(f32, f32, f32)>,
    Vec<((usize, usize, usize), (usize, usize, usize))>,
    Vec<(f32, f32)>)
{
    let f = File::open(path).unwrap();
    let buffer = BufReader::new(f);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut texture_vertices = Vec::new();

    let vre = Regex::new(r"v\s+([\d\-\.e]+)\s+([\d\-\.e]+)\s+([\d\-\.e]+)").unwrap();
    let fre = Regex::new(r"f\s+(\d*)/(\d*)[^ ]+\s+(\d*)/(\d*)[^ ]+\s+(\d*)/(\d*)[^ ]+").unwrap();
    let tre = Regex::new(r"vt\s+([\d\-\.e]+)\s+([\d\-\.e]+)").unwrap();

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
        }
    }

    (vertices, faces, texture_vertices)
}
