use crate::{hitable::{HitRecord, Hitable}, material::Material, ray::{self, Ray}, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = hit_sphere(&self.center, self.radius, ray);
        if t < t_min || t > t_max {
            return None;
        }
        let point = ray.at(t);
        let normal = point.sub(&self.center).divide(self.radius);
        Some(HitRecord { 
            t,
            point,
            front_face: ray.direction.dot(&normal) < 0.0,
            normal: if ray.direction.dot(&normal) < 0.0 {
                normal
            } else {
                normal.multiply(-1.0)
            },
            material: self.material.clone(),
        })
    }
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &ray::Ray) -> f32 {
    let oc = center.sub(&ray.origin);
    let a = ray.direction.length_squared();
    let h = ray.direction.dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}