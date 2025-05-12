use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

use image::{DynamicImage, ImageEncoder, codecs::png::PngEncoder};
use vec3::Vec3;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod scene1;
mod scene2;
mod scene3;
mod scene4;
mod scene5;
mod scene6;
mod sphere;
mod triangle;
mod util;
mod vec3;

fn main() {
    let (world, flen) = scene1::create_scene();
    let camera = camera::Camera::new(0.0, 1.0, 30.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 1000, 2000);
    let image = camera.render(&world, Vec3::new(0.5, 0.5, 0.7));
    encode_png(image.clone(), "scene1.png");

    let (world, flen) = scene2::create_scene();
    let camera = camera::Camera::new(6.0, 1.1, 40.0, Vec3::new(-0.9, 0.0, -0.3), Vec3::new(0.0, 0.0, -1.7), Vec3::new(0.0, 1.0, 0.0), 1000, 2000);
    let image = camera.render(&world, Vec3::new(0.5, 0.5, 0.7));
    encode_png(image.clone(), "scene2.png");

    let (world, flen) = scene3::create_scene();
    let camera = camera::Camera::new(0.0, 1.0, 50.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.5), Vec3::new(0.0, 1.0, 0.0), 1000, 2000);
    let image = camera.render(&world, Vec3::new(0.0, 0.0, 0.0));
    encode_png(image.clone(), "scene3.png");

    let (world, flen) = scene4::create_scene();
    let camera = camera::Camera::new(0.0, 1.0, 30.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.5), Vec3::new(0.0, 1.0, 0.0), 1000, 5000);
    let image = camera.render(&world, Vec3::new(0.0, 0.0, 0.0));
    encode_png(image.clone(), "scene4.png");

    let (world, flen) = scene5::create_scene();
    let camera = camera::Camera::new(0.0, 1.0, 30.0, Vec3::new(-1.5, 0.3, 0.0), Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 1.0, 0.0), 1000, 2000);
    let image = camera.render(&world, Vec3::new(0.5, 0.5, 0.7));
    encode_png(image.clone(), "scene5.png");

    let (world, flen) = scene5::create_scene();
    let camera = camera::Camera::new(5.0, 2.0, 30.0, Vec3::new(-1.5, 0.3, 0.0), Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 1.0, 0.0), 1000, 2000);
    let image = camera.render(&world, Vec3::new(0.5, 0.5, 0.7));
    encode_png(image.clone(), "scene5_blur.png");

    let (world, flen) = scene6::create_scene();
    let camera = camera::Camera::new(15.0, 1.28, 23.0, Vec3::new(0.7,0.1,-0.3), Vec3::new(0.0, 0.0, -1.5), Vec3::new(0.0, 1.0, 0.0), 1000, 5000);
    let image = camera.render(&world, Vec3::new(0.0, 0.0, 0.0));
    encode_png(image.clone(), "scene6.png");
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
