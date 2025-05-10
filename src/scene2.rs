
use rand::{Rng, SeedableRng};

use crate::{hitable_list::{self, HitableList}, material::{self, Material}, sphere, triangle, vec3::Vec3};

pub fn create_scene() -> (HitableList, f32) {
    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.3, -0.3, -1.0),
        0.2,
        Material::CheckerBoard(
            Box::new(Material::Light(Vec3::new(0.8, 0.2, 0.2).multiply(5.0))),
            Box::new(Material::Metallic(material::Metallic {
                albedo: Vec3::new(0.8, 1.0, 1.0),
                fuzz: 0.2,
            })),
            15.0,
        ),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-0.5, -0.3, -2.0),
        0.5,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(0.8, 0.6, 0.1),
            fuzz: 0.3,
        }),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(1.5, -0.3, -2.0),
        1.0,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(0.9, 0.9, 1.0),
            fuzz: 0.0,
        }),
    )));

    // seeded rand
    let mut seed = [0u8; 32];
    seed[0] = 0x30;
    let mut rng = rand_chacha::ChaCha8Rng::from_seed(seed);
    for i in 0..100 {
        let rand_position = Vec3::new(
            rng.random_range(-5.0..5.0),
            rng.random_range(-0.5..5.0),
            rng.random_range(-5.0..5.0),
        );
        
        // metal or glow
        let material = if rng.random_range(0..5) != 0 {
            Material::Metallic(material::Metallic {
                albedo: Vec3::new(0.5, 0.5, 0.5).add(&Vec3::new(
                    rng.random_range(0.0..1.0),
                    rng.random_range(0.0..1.0),
                    rng.random_range(0.0..1.0),
                ).multiply(0.5)),
                fuzz: rng.random_range(0.0..0.5),
            })
        } else {
            Material::Light(Vec3::new(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            ).multiply(3.0))
        };

        world.add(Box::new(sphere::Sphere::new(
            Vec3::new(
                rand_position.x,
                rand_position.y,
                rand_position.z,
            ),
            2.5 / (2.0 as f32).powf(rng.random_range(1.0..5.0)),
            material,
        )));
    }

    fn add_parallelogram(hitables: &mut HitableList, a: Vec3, b: Vec3, c: Vec3, material: Material) {
        hitables.add(Box::new(triangle::Triangle::new(a, b, c, material.clone())));
        hitables.add(Box::new(triangle::Triangle::new(a, c, c.add(&a.sub(&b)), material.clone())));
    }
    
    add_parallelogram(
        &mut world,
        Vec3::new(-1.0, 0.0, -2.0).multiply(500.0).sub(&Vec3::new(0.0, 0.5, 0.0)),
        Vec3::new(-1.0, 0.0, 2.0).multiply(500.0).sub(&Vec3::new(0.0, 0.5, 0.0)),
        Vec3::new(1.0, 0.0, 2.0).multiply(500.0).sub(&Vec3::new(0.0, 0.5, 0.0)),
        Material::CheckerBoard(
            Box::new(Material::Diffuse(Vec3::new(0.8, 0.8, 0.8))),
            Box::new(Material::Metallic(material::Metallic {
                albedo: Vec3::new(0.8, 1.0, 1.0),
                fuzz: 0.2,
            })),
            5.0,
        ),
    );

    (world, 0.9)
}