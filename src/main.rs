mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// To use encoder.set()
use ordered_float;
use png::HasParameters;

use crate::vec3::Vec3;

pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
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

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().dot(*r.direction());
        let b = 2.0 * oc.dot(*r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0.0 {
            return None;
        }
        let sol_pos = (-b + discriminant.sqrt()) / (2.0 * a);
        let sol_neg = (-b - discriminant.sqrt()) / (2.0 * a);
        let t: Option<f64> = {
            if sol_pos > t_min && sol_pos < t_max {
                Some(sol_pos)
            } else if sol_neg > t_min && sol_neg < t_max {
                Some(sol_neg)
            } else {
                None
            }
        };
        match t {
            Some(t_val) => {
                let p = r.point_at_parameter(t_val);
                Some(HitRecord {
                    t: t_val,
                    p,
                    normal: (p - self.center) / self.radius,
                })
            }
            None => None,
        }
    }
}

pub struct World {
    hittables: Vec<Box<Hittable>>,
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hittables
            .iter()
            .filter_map(|h| h.hit(&r, t_min, t_max))
            .min_by_key(|r| ordered_float::OrderedFloat(r.t))
    }
}

fn color(r: &Ray, world: &Hittable) -> Vec3 {
    match world.hit(&r, 0., 99999999999.) {
        Some(hit_record) => {
            0.5 * Vec3(
                hit_record.normal.x() + 1.,
                hit_record.normal.y() + 1.,
                hit_record.normal.z() + 1.,
            )
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let num_samples_per_pixel = 100;
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);
    let world = World {
        hittables: vec![
            Box::new(Sphere {
                center: Vec3(0., -100.5, -1.),
                radius: 100.,
            }),
            Box::new(Sphere {
                center: Vec3(0., 0., -1.),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3(0.4, 0.5, -2.),
                radius: 1.,
            }),
        ],
    };

    let mut img_data = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0., 0., 0.);
            for k in 0..num_samples_per_pixel {
                let u = ((i as f64) + rand::random::<f64>()) / (nx as f64);
                let v = ((j as f64) + rand::random::<f64>()) / (ny as f64);
                let r = Ray {
                    a: origin,
                    b: lower_left_corner + u * horizontal + v * vertical,
                };
                col = col + color(&r, &world);
            }
            col = col / (num_samples_per_pixel as f64);
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
