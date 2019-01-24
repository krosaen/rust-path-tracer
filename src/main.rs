use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::HasParameters;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut img_data = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.2;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
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
