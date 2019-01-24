use nalgebra::core::{Vector2, Vector3, Vector4};
use nalgebra::geometry::Point3;
use img::Img;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use super::{barycentric, max, min};

pub trait Shader {
    fn vertex(&mut self, face_index: usize, vertex_index:usize) /*face_vertex: Point3<f32>, face_vertex_normal: Vector3<f32>, vertice_index: usize)*/ -> Point3<f32>;
    fn fragment(&self, bary_coords: (f32, f32, f32)) -> ((u8, u8, u8), bool);
}

pub fn triangle<S: Shader>(
    p0: Vector3<f32>,
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    shader: &S,
    img: &mut Img,
    zbuffer: &mut HashMap<(i32, i32), f32>)
{
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

                let key = (x as i32, y as i32);
                if !zbuffer.contains_key(&key) || zbuffer[&key] < z {
                    zbuffer.insert(key, z);
                    let (color, discard) = shader.fragment(bary);
                    if !discard {
                        img.set(x as u32, y as u32, color);
                    }
                }
            }
        }
    }
}
