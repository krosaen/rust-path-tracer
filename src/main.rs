mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// To use encoder.set()
use png::HasParameters;

use crate::vec3::Vec3;

struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    fn origin(&self) -> &Vec3 {
        &self.a
    }
    fn direction(&self) -> &Vec3 {
        &self.b
    }
    fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + (self.b * t)
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - center;
    let a = r.direction().dot(*r.direction());
    let b = 2.0 * oc.dot(*r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = r.point_at_parameter(t).unit() - Vec3(0.0, 0.0, -1.0);
        return 0.5 * Vec3(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    let mut img_data = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray {
                a: origin,
                b: lower_left_corner + u * horizontal + v * vertical,
            };
            let col = color(&r);
            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;
            img_data.push(ir);
            img_data.push(ig);
            img_data.push(ib);
            img_data.push(255);
        }
    }
    save_png(&img_data, "test.png");
}

fn save_png(data: &[u8], name: &str) {
    let file = File::create(Path::new(name)).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 200, 100);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
    println!("saved {}", name);
}
