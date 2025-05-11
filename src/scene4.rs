use rand::{Rng, SeedableRng};

use crate::{
    hitable_list::{self, HitableList},
    material::{self, Material},
    sphere, triangle,
    vec3::Vec3,
};

pub fn create_scene() -> (HitableList, f32) {
    let mut world = hitable_list::HitableList::new();

    fn rect(
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

    rect(
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
 
    rect(
        &mut world,
        Vec3::new(-0.2, 0.499, -0.8),
        Vec3::new(-0.2, 0.499, -0.6),
        Vec3::new(0.2, 0.499, -0.6),
        Material::Light(
            Vec3::new(1.0, 1.0, 1.0).multiply(10.0),
        ),
    ); 

    rect(
        &mut world,
        Vec3::new(0.5, -0.5, -1.0),
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),
        Material::Diffuse(Vec3::new(0.0, 1.0, 0.0)),
    ); 

    rect(
        &mut world,
        Vec3::new(-0.5, -0.5, -1.0),
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(-0.5, 0.5, 0.0),
        Material::Diffuse(Vec3::new(1.0, 0.0, 0.0)),
    ); 

    rect(
        &mut world,
        Vec3::new(-0.5, 0.5, -1.0),
        Vec3::new(-0.5, 0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),
        Material::Diffuse(Vec3::new(1.0, 1.0, 1.0)),
    ); 


    rect(
        &mut world,
        Vec3::new(-0.5, 0.5, -1.0),
        Vec3::new(0.5, 0.5, -1.0),
        Vec3::new(0.5, -0.5, -1.0),
        Material::Diffuse(Vec3::new(1.0, 1.0, 1.0)),
    ); 

    // in a volume, add a lot of spheres
    let mut rng = rand::rngs::StdRng::from_seed([0; 32]);
    for _ in 0..200 {
        let rand_position = Vec3::new(
            rng.gen_range(-0.15..0.15),
            rng.gen_range(-0.15..0.15),
            rng.gen_range(-0.7..-0.5),
        );

        world.add(Box::new(sphere::Sphere::new(
            rand_position,
            0.05,
            Material::Diffuse(Vec3::new(1.0, 1.0, 1.0)),
        )));
    }

    (world, 0.9)
}
