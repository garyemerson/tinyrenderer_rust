extern crate regex;
extern crate image;

mod img;
use img::Img;

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;
use std::mem;

fn main() {
    let width: f32 = 800.0;
    let height: f32 = 800.0;
    let mut img = Img::new(width as u32, height as u32);

    // for _ in 0..1000000 {
    //     line4(50, 50, 50, 60, &mut img, (255, 255, 255));
    //     line4(50, 50, 50, 40, &mut img, (255, 255, 255));
    //     line4(50, 50, 60, 60, &mut img, (255, 255, 255));
    //     line4(50, 50, 60, 50, &mut img, (255, 255, 255));
    //     line4(50, 50, 40, 50, &mut img, (255, 255, 255));
    //     line4(50, 50, 60, 40, &mut img, (255, 255, 255));
    //     line4(50, 50, 40, 60, &mut img, (255, 255, 255));
    //     line4(50, 50, 40, 40, &mut img, (255, 255, 255));
    // }

    let (vertices, faces) = parse_obj_file("/Users/Garrett/Dropbox/Files/workspaces/tinyrenderer_rust/african_head.obj");
    println!("{} vertices, {} faces", vertices.len(), faces.len());
    for f in faces {
        line4(
            ((vertices[(f.0 - 1) as usize].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[(f.0 - 1) as usize].1 + 1.0) * height / 2.0).floor() as u32,
            ((vertices[(f.1 - 1) as usize].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[(f.1 - 1) as usize].1 + 1.0) * height / 2.0).floor() as u32,
            &mut img, (255, 255, 255));
        line4(
            ((vertices[(f.1 - 1) as usize].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[(f.1 - 1) as usize].1 + 1.0) * height / 2.0).floor() as u32,
            ((vertices[(f.2 - 1) as usize].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[(f.2 - 1) as usize].1 + 1.0) * height / 2.0).floor() as u32,
            &mut img, (255, 255, 255));
        line4(
            ((vertices[(f.2 - 1) as usize].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[(f.2 - 1) as usize].1 + 1.0) * height / 2.0).floor() as u32,
            ((vertices[(f.0 - 1) as usize].0 + 1.0) * width / 2.0).floor() as u32,
            ((vertices[(f.0 - 1) as usize].1 + 1.0) * height / 2.0).floor() as u32,
            &mut img, (255, 255, 255));
    }

    img.flip_vertical();
    img.save("output.png");
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

    println!("drawing line ({}, {}) ({}, {})", x0, y0, x1, y1);

    let dx = ((x0 as i32) - (x1 as i32)).abs();
    let dy = ((y0 as i32) - (y1 as i32)).abs();
    let mut y = y0;
    let mut error2: i32 = 0;

    for x in x0..(x1 + 1) {
        if steep {img.put(y as u32, x as u32, color); } else {img.put(x as u32, y as u32, color);}

        if 2 * (error2 + dy) < dx {
            error2 = error2 + dy;
        } else {
            // TODO: somehow y is becoming negative with diablo3 wirefram
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
            img.put(y, x, color);
        } else {
            img.put(x, y, color);
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
            img.put(y.round() as u32, x, color);
        } else {
            img.put(x, y.round() as u32, color);
        }
    }
}

fn line(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    for step in 0..100 {
        let t = (step as f64) / 100f64;
        let x = (x0 as f64) * (1f64 - t) + (x1 as f64) * t;
        let y = (y0 as f64) * (1f64 - t) + (y1 as f64) * t;
        img.put(x.round() as u32, y.round() as u32, color);
    }
}

// Returns (vertices, faces)
fn parse_obj_file(path: &str) -> (Vec<(f32, f32, f32)>, Vec<(u32, u32, u32)>) {
    let f = File::open(path).unwrap();
    let mut buffer = BufReader::new(f);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    let vre = Regex::new(r"v ([\d\-\.e]+) ([\d\-\.e]+) ([\d\-\.e]+)").unwrap();
    let fre = Regex::new(r"f (\d+)/[^ ]+ (\d+)/[^ ]+ (\d+)/[^ ]+").unwrap();

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
                let f2 = &cap[2].trim();
                let f3 = &cap[3].trim();
                let f = (f1.parse::<u32>().unwrap(), f2.parse::<u32>().unwrap(), f3.parse::<u32>().unwrap());
                faces.push(f);
            }
        }
    }

    (vertices, faces)
}
