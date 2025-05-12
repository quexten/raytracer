use rand::{Rng, SeedableRng};

use crate::{
    hitable_list::{self, HitableList},
    material::{self, Material},
    sphere, triangle,
    vec3::Vec3,
};

pub fn create_scene() -> (HitableList, f32) {
    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(1.0, -0.1, -1.5),
        0.3,
        Material::Light(Vec3::new(1.0, 1.0, 1.0).multiply(4.0)),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-1.0, -0.2, -1.5),
        0.3,
        Material::Crazy()
    )));

    fn add_parallelogram(
        hitables: &mut HitableList,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        material: Material,
    ) {
        hitables.add(Box::new(triangle::Triangle::new(a, b, c, material.clone())));
        hitables.add(Box::new(triangle::Triangle::new(
            a,
            c,
            c.add(&a.sub(&b)),
            material.clone(),
        )));
    }

    add_parallelogram(
        &mut world,
        Vec3::new(-1.0, 0.0, -2.0)
            .multiply(500.0)
            .sub(&Vec3::new(0.0, 0.5, 0.0)),
        Vec3::new(-1.0, 0.0, 2.0)
            .multiply(500.0)
            .sub(&Vec3::new(0.0, 0.5, 0.0)),
        Vec3::new(1.0, 0.0, 2.0)
            .multiply(500.0)
            .sub(&Vec3::new(0.0, 0.5, 0.0)),
        Material::CheckerBoard(
            Box::new(Material::Diffuse(Vec3::new(1.0, 1.0, 1.0))),
            Box::new(Material::Diffuse(Vec3::new(0.0, 0.0, 0.0))),
            8.0,
        ),
    );

    // wall middle
    add_parallelogram(
        &mut world,
        Vec3::new(0.6, -0.5, -2.0),
        Vec3::new(0.6, -0.5, -1.0),
        Vec3::new(0.6, 1.0, -1.0),
        Material::Diffuse(Vec3::new(1.0, 1.0, 1.0)),
    ); 

    (world, 0.9)
}
