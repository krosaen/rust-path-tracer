use std::fs::File;
use std::io::BufWriter;
use std::ops;
use std::path::Path;
// To use encoder.set()
use png::HasParameters;

struct Vec3(f64, f64, f64);

impl Vec3 {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
    fn r(&self) -> f64 {
        self.0
    }
    fn g(&self) -> f64 {
        self.1
    }
    fn b(&self) -> f64 {
        self.2
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

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
    fn point_at_parameter(self, t: f64) -> Vec3 {
        self.a + (self.b * t)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut img_data = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let color = Vec3((i as f64) / (nx as f64), (j as f64) / (ny as f64), 0.2);
            let ir = (255.99 * color.r()) as u8;
            let ig = (255.99 * color.g()) as u8;
            let ib = (255.99 * color.b()) as u8;
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
