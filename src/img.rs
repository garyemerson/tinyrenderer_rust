use std::fs::File;
use std::path::Path;
use image::{self, ImageBuffer, Rgb, imageops};

pub struct Img {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    w: u32,
    h: u32,
}

impl Img {
    pub fn new(w: u32, h: u32) -> Img {
        let mut buf = ImageBuffer::new(w, h);
        buf.put_pixel(0, 0, Rgb([0 as u8, 0 as u8, 0 as u8]));
        Img{
            buf: buf,
            w: w,
            h: h,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: (u8, u8, u8)) {
        if x < self.w && y < self.h {
            self.buf.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
        }
    }

    pub fn flip_vertical(&mut self) {
        self.buf = imageops::flip_vertical(&self.buf);
    }

    pub fn save(self, path: &str) {
        let f = &mut File::create(&Path::new(path)).unwrap();
        image::ImageRgb8(self.buf).save(f, image::PNG).unwrap();
    }
}
