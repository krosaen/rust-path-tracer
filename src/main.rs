mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// To use encoder.set()
use ordered_float;
use png::HasParameters;

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
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

fn color(r: Ray, world: &Hittable) -> Vec3 {
    match world.hit(&r, 0.001, std::f64::MAX) {
        Some(hit_record) => {
            // bounce in a random new direction
            let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
            0.5 * color(
                Ray {
                    a: hit_record.p,
                    b: target - hit_record.p,
                },
                world,
            )
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p =
            2.0 * Vec3(
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ) - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical,
        }
    }
}

fn main() {
    let nx = 400;
    let ny = 200;
    let num_samples_per_pixel = 100;
    let cam = Camera {
        origin: Vec3(0.0, 0.0, 0.0),
        lower_left_corner: Vec3(-2.0, -1.0, -1.0),
        horizontal: Vec3(4.0, 0.0, 0.0),
        vertical: Vec3(0.0, 2.0, 0.0),
    };
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
            // Box::new(Sphere {
            //     center: Vec3(0.4, 0.5, -2.),
            //     radius: 1.,
            // }),
        ],
    };

    let mut img_data = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0., 0., 0.);
            for _k in 0..num_samples_per_pixel {
                let u = ((i as f64) + rand::random::<f64>()) / (nx as f64);
                let v = ((j as f64) + rand::random::<f64>()) / (ny as f64);
                let r = cam.get_ray(u, v);
                col = col + color(r, &world);
            }
            col = col / (num_samples_per_pixel as f64);
            let ir = (255.99 * col.r().sqrt()) as u8;  // sqrt for gamma 2
            let ig = (255.99 * col.g().sqrt()) as u8;
            let ib = (255.99 * col.b().sqrt()) as u8;
            img_data.push(ir);
            img_data.push(ig);
            img_data.push(ib);
            img_data.push(255);
        }
    }
    save_png(&img_data, "test.png", nx, ny);
}

fn save_png(data: &[u8], name: &str, width: i32, height: i32) {
    let file = File::create(Path::new(name)).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
    println!("saved {}", name);
}
