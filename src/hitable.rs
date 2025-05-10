use dyn_clone::DynClone;

use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
}

pub trait Hitable: DynClone + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}