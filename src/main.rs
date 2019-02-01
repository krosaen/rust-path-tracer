use std::fs::File;
use std::io::BufWriter;
use std::num;
use std::ops;
use std::path::Path;
// To use encoder.set()
use png::HasParameters;

#[derive(Debug, Copy, Clone)]
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
    fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    fn unit(&self) -> Self {
        *self / self.length()
    }
    fn dot(&self, other: Self) -> f64 {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
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

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = *r.origin() - center;
    let a = r.direction().dot(*r.direction());
    let b = 2.0 * oc.dot(*r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    if (hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r)) {
        return Vec3(1.0, 0.0, 0.0);
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
