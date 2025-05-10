use crate::{hitable_list::{self, HitableList}, material::{self, Material}, sphere, triangle, vec3::Vec3};

pub fn create_scene() -> (HitableList, f32) {
    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.3, -0.3, -1.0),
        0.2,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 0.2, 0.2),
            fuzz: 0.4,
        }),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-0.4, -0.5, -1.3),
        0.3,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(0.8, 1.0, 1.0),
            fuzz: 0.0,
        }),
    )));


    fn add_parallelogram(hitables: &mut HitableList, a: Vec3, b: Vec3, c: Vec3, material: Material) {
        hitables.add(Box::new(triangle::Triangle::new(a, b, c, material.clone())));
        hitables.add(Box::new(triangle::Triangle::new(a, c, c.add(&a.sub(&b)), material)));
    }
    
    add_parallelogram(
        &mut world,
        Vec3::new(-1.0, -0.5, -2.0),
        Vec3::new(-1.0, -0.5, 2.0),
        Vec3::new(1.0, -0.5, 2.0),
        Material::Diffuse(Vec3::new(0.8, 0.8, 0.8)),
    );

    add_parallelogram(
        &mut world,
        Vec3::new(0.5, -0.5, -1.25),
        Vec3::new(-0.5, -0.5, -1.25),
        Vec3::new(-0.5, 0.5, -1.25),
        Material::CheckerBoard(
            Box::new(Material::Light(Vec3::new(0.8, 0.2, 0.2).multiply(3.0))),
            Box::new(Material::Diffuse(Vec3::new(0.0, 0.0, 0.0))),
            15.0,
        ),
    );
    add_parallelogram(
        &mut world,
        Vec3::new(0.5, 0.5, -1.25),
        Vec3::new(-0.5, 0.5, -1.25),
        Vec3::new(-0.5, 0.5, -0.75),
        Material::Diffuse(Vec3::new(0.8, 0.8, 0.8)),
    );
    // right wall
    add_parallelogram(
        &mut world,
        Vec3::new(0.5, -0.5, -1.25),
        Vec3::new(0.5, -0.5, -0.75),
        Vec3::new(0.5, 0.5, -0.75),
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.2,
        }),
    );
    // left wall
    add_parallelogram(
        &mut world,
        Vec3::new(-0.5, -0.5, -1.25),
        Vec3::new(-0.5, -0.5, -0.75),
        Vec3::new(-0.5, 0.5, -0.75),
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(0.4, 0.4, 0.4),
            fuzz: 0.5,
        }),
    );

    (world, 1.6)
}