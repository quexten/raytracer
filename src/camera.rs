use std::sync::{Arc, Mutex};

use colorgrad::Gradient;
use image::{DynamicImage, GenericImage};
use indicatif::ProgressBar;
use noise::{NoiseFn, Perlin};
use rand::rand_core::le;

use crate::{
    hitable::{self, Hitable},
    hitable_list::{self, HitableList},
    material::Material,
    ray,
    util::random_double_range,
    vec3::{self, Vec3},
};

const MAX_DEPTH: u32 = 10;
const THREADS: usize = 16;

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_00_location: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub camera_center: Vec3,
    pub bar: Arc<Mutex<ProgressBar>>,
    pub samples_per_pixel: u32,
    pub look_at: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    defocus_dist_u: Vec3,
    defocus_dist_v: Vec3,
}

impl Camera {
    pub fn new(defocus_angle: f32, focus_distance: f32, fov: f32, look_from: Vec3, look_at: Vec3, up: Vec3, width: u32, samples_per_pixel: u32) -> Self {
        let aspect_ratio = 1.0 / 1.0;
        let image_width = width;
        let image_height = (image_width as f32 / aspect_ratio) as u32;

        let w = look_from.sub(&look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u).normalize();

        let viewport_height = 2.0 * (fov * std::f32::consts::PI / 180.0).tan() * focus_distance;
        let viewport_width = aspect_ratio * viewport_height;
        let camera_center = look_from;

        let viewport_u = &u.multiply(viewport_width);
        let viewport_v = &v.multiply(-viewport_height);

        let pixel_delta_u = viewport_u.divide(image_width as f32);
        let pixel_delta_v = viewport_v.divide(image_height as f32);

        let viewport_upper_left = camera_center
            .sub(&w.multiply(focus_distance))
            .add(&viewport_u.multiply(-0.5))
            .add(&viewport_v.multiply(-0.5));
        let pixel_00_location = viewport_upper_left
            .add(&pixel_delta_u.multiply(0.5))
            .add(&pixel_delta_v.multiply(0.5));

        let bar = Arc::new(Mutex::new(ProgressBar::new(
            image_height as u64 * THREADS as u64,
        )));


        let angle_radian = defocus_angle * std::f32::consts::PI / 180.0;
        let defocus_radius = focus_distance * (angle_radian / 2.0).tan();
        let defocus_dist_u = u.multiply(defocus_radius);
        let defocus_dist_v = v.multiply(defocus_radius);

        Camera {
            image_width,
            image_height,
            pixel_00_location,
            pixel_delta_u,
            pixel_delta_v,
            camera_center,
            bar,
            samples_per_pixel,
            look_at,
            up,
            fov,
            u,
            v,
            w,
            defocus_angle,
            focus_distance,
            defocus_dist_u,
            defocus_dist_v,
        }
    }

    pub fn render(&self, world: &HitableList, ambient_light: Vec3) -> Arc<Mutex<DynamicImage>> {
        let dynamic_image = DynamicImage::new_rgb8(self.image_width, self.image_height);

        // tasks
        let mut ranges: Vec<(usize, usize)> = vec![];
        for i in 0..THREADS {
            let start = (i * (self.image_width as usize / THREADS)) as usize;
            let end = if i == THREADS - 1 {
                self.image_width as usize
            } else {
                ((i + 1) * (self.image_width as usize / THREADS)) as usize
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
            samples_per_pixel: u32,
            bar: Arc<Mutex<ProgressBar>>,
            defocus_angle: f32,
            defocus_dist_u: Vec3,
            defocus_dist_v: Vec3,
            ambient_light: Vec3
        ) {
            for j in 0..image_height {
                for i in start..end {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let (offset_x, offset_y) = (
                            random_double_range(-0.75, 0.75),
                            random_double_range(-0.75, 0.75),
                        );
                        let pixel_center = pixel_00_location
                            .add(&pixel_delta_u.multiply(i as f32 + offset_x))
                            .add(&pixel_delta_v.multiply(j as f32 + offset_y));
                        let ray_origin = if defocus_angle < 0.0 {
                            camera_center
                        } else {
                            let random = Vec3::random_in_unit_disk();
                            let sample = camera_center
                                .add(&defocus_dist_u.multiply(random.x))
                                .add(&defocus_dist_v.multiply(random.y));
                            sample
                        };
                        let ray_direction = pixel_center.sub(&ray_origin);
                        let ray = ray::Ray::new(ray_origin, ray_direction);
                        let ray_color = ray_color(&ray, world, 0, ambient_light);
                        pixel_color = pixel_color.add(&ray_color);
                    }
                    pixel_color = pixel_color.divide(samples_per_pixel as f32);
                    // linear to gamma correction
                    pixel_color = Vec3::new(
                        pixel_color.x.sqrt(),
                        pixel_color.y.sqrt(),
                        pixel_color.z.sqrt(),
                    );

                    let mut dynamic_image = image.lock().unwrap();
                    write_color(
                        &mut dynamic_image,
                        image::Rgba([
                            (pixel_color.x * 255.0) as u8,
                            (pixel_color.y * 255.0) as u8,
                            (pixel_color.z * 255.0) as u8,
                            255,
                        ]),
                        i as u32,
                        j as u32,
                    );
                }
                bar.lock().unwrap().inc(1);
            }
        }

        println!("Starting rendering");
        let mut handles = vec![];
        for (start, end) in ranges {
            let image_clone = image_mutex.clone();
            let world_clone = world.clone();
            let pixel_00_location_clone = self.pixel_00_location.clone();
            let pixel_delta_u_clone = self.pixel_delta_u.clone();
            let pixel_delta_v_clone = self.pixel_delta_v.clone();
            let camera_center_clone = self.camera_center.clone();
            let bar = self.bar.clone();
            let image_height_clone = self.image_height.clone();
            let samples_per_pixel_clone = self.samples_per_pixel.clone();
            let defocus_angle_clone = self.defocus_angle.clone();
            let defocus_dist_u_clone = self.defocus_dist_u.clone();
            let defocus_dist_v_clone = self.defocus_dist_v.clone();
            let ambient_light_clone = ambient_light.clone();

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
                    image_height_clone,
                    samples_per_pixel_clone,
                    bar,
                    defocus_angle_clone,
                    defocus_dist_u_clone,
                    defocus_dist_v_clone,
                    ambient_light_clone,
                );
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        self.bar.lock().unwrap().finish();
        image_mutex.clone()
    }
}

fn write_color(img: &mut DynamicImage, pixel: image::Rgba<u8>, x: u32, y: u32) {
    img.put_pixel(x, y, pixel);
}

fn ray_color(ray: &ray::Ray, hitables: &HitableList, depth: u32, ambient_light: Vec3) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = hitables.hit(ray, 0.001, f32::INFINITY) {
        let material = hit_record.material.clone();
        let color = handle_material(&hit_record, &material, hitables, depth, ray, ambient_light);
        return color;
    }

    ambient_light
}

fn handle_material(
    hit_record: &hitable::HitRecord,
    material: &Material,
    hitables: &hitable_list::HitableList,
    depth: u32,
    ray: &ray::Ray,
    ambient_light: Vec3,
) -> Vec3 {
    match material {
        Material::Light(color) => {
            return *color;
        }
        Material::Metallic(metal) => {
            let diffuse = if (metal.diffuse) {
            
             handle_material(
                hit_record,
                &Material::Diffuse(metal.albedo),
                hitables,
                depth,
                ray,
                ambient_light,
             )
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };

            let reflected = ray.direction.reflect(&hit_record.normal);
            let scattered = ray::Ray::new(
                hit_record.point,
                reflected.add(&Vec3::random_unit().multiply(metal.fuzz)),
            );
            if scattered.direction.dot(&hit_record.normal) > 0.0 {
                let reflected = ray_color(&scattered, hitables, depth + 1, ambient_light);
                let pairwise_multiply = Vec3::new(
                    reflected.x * metal.albedo.x,
                    reflected.y * metal.albedo.y,
                    reflected.z * metal.albedo.z,
                );
                return pairwise_multiply.multiply(0.5).add(&diffuse.multiply(0.5));
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }
        Material::Diffuse(color) => {
            let next_ray_direction = hit_record.normal.add(&Vec3::random_unit());
            let reflected = ray_color(
                &ray::Ray::new(hit_record.point, next_ray_direction),
                hitables,
                depth + 1,
                ambient_light,
            );
            let pairwise_multiply = Vec3::new(
                reflected.x * color.x,
                reflected.y * color.y,
                reflected.z * color.z,
            );
            return pairwise_multiply;
        }
        Material::CheckerBoard(mat1, mat2, length) => {
            let material = if (hit_record.point.x * length).floor().abs() as i32 % 2
                == (hit_record.point.z * length).floor().abs() as i32 % 2
            {
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
            handle_material(hit_record, &material, hitables, depth, ray, ambient_light)
        },
        Material::Crazy() => {
            let noise = Perlin::new(4);
            let factor = 20.0;
            let noise_at: f64 = noise.get([hit_record.point.x as f64 * factor, hit_record.point.y as f64 * factor, hit_record.point.z as f64 * factor]);
            let distance = noise_at - 0.5;
            let noise_at = (distance.abs().exp() - 1.0) / (distance.abs().exp() + 1.0);
           
            let rgb = lava_gradient(
                (noise_at as f32).clamp(0.0, 1.0)
            );

            return Vec3::new(rgb.x, rgb.y, rgb.z).multiply(1.0);
        },
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 240.0 {
        (0.0, c, x)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Vec3::new(r + m, g + m, b + m)
}

fn lava_gradient(x: f32) -> Vec3 {
    let a = colorgrad::preset::rainbow();
    let rgb = a.at(x).to_rgba8();
    Vec3::new(
        rgb[0] as f32 / 255.0,
        rgb[1] as f32 / 255.0,
        rgb[2] as f32 / 255.0,
    )
}