use std::{fs::File, io::BufWriter, sync::{Arc, Mutex}};

use hitable::Hitable;
use hitable_list::HitableList;
use image::{codecs::png::PngEncoder, DynamicImage, GenericImage, ImageEncoder};
use indicatif::ProgressBar;
use material::Material;
use util::random_double_range;
use vec3::Vec3;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod triangle;
mod hitable_list;
mod util;
mod material;
mod scene1;
mod scene2;

const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 10;
const THREADS: usize = 8;

fn main() {
    let (world, flen) = scene1::create_scene();
    let aspect_ratio = 1.0 / 1.0;
    let image_width = 1000;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // camera
    let focal_length = flen;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = vec3::Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.divide(image_width as f32);
    let pixel_delta_v = viewport_v.divide(image_height as f32);

    let viewport_upper_left = camera_center
        .sub(&Vec3::new(0.0,0.0,focal_length))
        .sub(&viewport_u.divide(2.0))
        .sub(&viewport_v.divide(2.0));
    let pixel_00_location = viewport_upper_left
        .add(&pixel_delta_u.multiply(0.5))
        .add(&pixel_delta_v.multiply(0.5));

    let dynamic_image = DynamicImage::new_rgb8(image_width, image_height);
    let bar = Arc::new(Mutex::new(ProgressBar::new(image_height as u64 * THREADS as u64)));



    // tasks
    let mut ranges: Vec<(usize, usize)> = vec![];
    for i in 0..THREADS {
        let start = (i * (image_width as usize / THREADS)) as usize;
        let end = if i == THREADS - 1 {
            image_width as usize
        } else {
            ((i + 1) * (image_width as usize / THREADS)) as usize
        };
        ranges.push((start, end));
    }
    let image_mutex = Arc::new(Mutex::new(dynamic_image));

    fn render_chunk(
        image: Arc<Mutex<DynamicImage>>,
        world: &HitableList,
        pixel_00_location: Vec3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        start: usize,
        end: usize,
        camera_center: Vec3,
        image_height: u32,
        bar: Arc<Mutex<ProgressBar>>,
     ) {
        for j in 0..image_height {
            for i in start..end {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let (offset_x, offset_y) = (
                        random_double_range(-0.75, 0.75),
                        random_double_range(-0.75, 0.75),
                    );
                    let pixel_center = pixel_00_location
                        .add(&pixel_delta_u.multiply(i as f32 + offset_x))
                        .add(&pixel_delta_v.multiply(j as f32 + offset_y));
                    let ray_direction = pixel_center.sub(&camera_center);
                    let ray = ray::Ray::new(camera_center, ray_direction);
                    let ray_color = ray_color(&ray, world, 0);
                    pixel_color = pixel_color.add(&ray_color);
                }
                pixel_color = pixel_color.divide(SAMPLES_PER_PIXEL as f32);
                // linear to gamma correction
                pixel_color = Vec3::new(
                    pixel_color.x.sqrt(),
                    pixel_color.y.sqrt(),
                    pixel_color.z.sqrt(),
                );

                let mut dynamic_image = image.lock().unwrap();
                write_color(
                    &mut dynamic_image,
                    image::Rgba([(pixel_color.x * 255.0) as u8, (pixel_color.y * 255.0) as u8, (pixel_color.z * 255.0) as u8, 255]),
                    i as u32,
                    j as u32,
                );
            }
            bar.lock().unwrap().inc(1);
        }
    }

    let start = std::time::Instant::now();
    println!("Starting rendering");
    let mut handles = vec![];
    for (start, end) in ranges {
        let image_clone = image_mutex.clone();
        let world_clone = world.clone();
        let pixel_00_location_clone = pixel_00_location.clone();
        let pixel_delta_u_clone = pixel_delta_u.clone();
        let pixel_delta_v_clone = pixel_delta_v.clone();
        let camera_center_clone = camera_center.clone();
        let bar = bar.clone();

        let handle = std::thread::spawn(move || {
            render_chunk(
                image_clone,
                &world_clone,
                pixel_00_location_clone,
                pixel_delta_u_clone,
                pixel_delta_v_clone,
                start,
                end,
                camera_center_clone,
                image_height,
                bar,
            );
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    bar.lock().unwrap().finish();
    let elapsed = start.elapsed();
    println!("Rendering took: {:?} on {} threads", elapsed, THREADS);
    println!("Total rays: {}", image_width * image_height * SAMPLES_PER_PIXEL);
    println!("Total rays per second: {}", (image_width * image_height * SAMPLES_PER_PIXEL) as f32 / elapsed.as_secs_f32());
    encode_png(image_mutex.clone());
}


fn write_color(
    img: &mut DynamicImage,
    pixel: image::Rgba<u8>,
    x: u32,
    y: u32,
) {
    img.put_pixel(x, y, pixel);
}

fn encode_png(img: Arc<Mutex<DynamicImage>>) {
    let img = img.lock().unwrap();
    let file = File::create("icon.png").unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = PngEncoder::new(buff);
    encoder.write_image(
        img.as_rgb8().unwrap(),
        img.width(),
        img.height(),
        image::ExtendedColorType::Rgb8,
    ).unwrap();
}

fn ray_color(ray: &ray::Ray, hitables: &HitableList, depth: u32) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = hitables.hit(ray, 0.001, f32::INFINITY) {
        let material = hit_record.material.clone();
        let color = handle_material(&hit_record, &material, hitables, depth, ray);
        return color;
    }

    let t = 0.5 * (-ray.direction.y + 1.0);
    let white = Vec3::new(0.2, 0.2, 0.2);
    white.multiply(t).add(&Vec3::new(0.6, 0.8, 1.0).multiply(1.0 - t)).multiply(0.2)
}

fn handle_material(
    hit_record: &hitable::HitRecord,
    material: &Material,
    hitables: &hitable_list::HitableList,
    depth: u32,
    ray: &ray::Ray,
) -> Vec3 {
    match material {
        Material::Light(color) => {
            return *color;
        },
        Material::Metallic(metal) => {
            let reflected = ray.direction.reflect(&hit_record.normal);
            let scattered = ray::Ray::new(hit_record.point, reflected.add(&Vec3::random_unit().multiply(metal.fuzz)));
            if scattered.direction.dot(&hit_record.normal) > 0.0 {
                let reflected = ray_color(&scattered, hitables, depth + 1);
                let pairwise_multiply = Vec3::new(
                    reflected.x * metal.albedo.x,
                    reflected.y * metal.albedo.y,
                    reflected.z * metal.albedo.z,
                );
                return pairwise_multiply.multiply(0.5);

            }
            return Vec3::new(0.0, 0.0, 0.0);
        },
        Material::Diffuse(color) => {
            let next_ray_direction = hit_record.normal.add(&Vec3::random_unit());
            let reflected = ray_color(&ray::Ray::new(hit_record.point, next_ray_direction), hitables, depth + 1);
            let pairwise_multiply = Vec3::new(
                reflected.x * color.x,
                reflected.y * color.y,
                reflected.z * color.z,
            );
            return pairwise_multiply;
        },
        Material::CheckerBoard(mat1, mat2, length) => {
            let material = if (hit_record.point.x * length).floor().abs() as i32 % 2 == (hit_record.point.z * length).floor().abs() as i32 % 2 {
                if (hit_record.point.y * length).floor().abs() as i32 % 2 == 0 {
                    mat1
                } else {
                    mat2
                }
            } else {
                if (hit_record.point.y * length).floor().abs() as i32 % 2 == 0 {
                    mat2
                } else {
                    mat1
                }
            };
            handle_material(hit_record,  &material, hitables, depth, ray)
        }
    }
}

