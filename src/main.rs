use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

use image::{DynamicImage, ImageEncoder, codecs::png::PngEncoder};

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod scene1;
mod scene2;
mod scene3;
mod scene4;
mod sphere;
mod triangle;
mod util;
mod vec3;

fn main() {
    // let (world, flen) = scene1::create_scene();
    // let camera = camera::Camera::new(flen, 1000, 1000);
    // let image = camera.render(&world);
    // encode_png(image.clone(), "scene1.png");

    let (world, flen) = scene2::create_scene();
    let camera = camera::Camera::new(flen, 1000, 1000);
    let image = camera.render(&world);
    encode_png(image.clone(), "scene2.png");

    // let (world, flen) = scene3::create_scene();
    // let camera = camera::Camera::new(flen, 1000, 1000);
    // let image = camera.render(&world);
    // encode_png(image.clone(), "scene3.png");

    // let (world, flen) = scene4::create_scene();
    // let camera = camera::Camera::new(flen, 1024, 2000);
    // let image = camera.render(&world);
    // encode_png(image.clone(), "scene4.png");
}

fn encode_png(img: Arc<Mutex<DynamicImage>>, filename: &str) {
    let img = img.lock().unwrap();
    let file = File::create(filename).unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = PngEncoder::new(buff);
    encoder
        .write_image(
            img.as_rgb8().unwrap(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();
}
