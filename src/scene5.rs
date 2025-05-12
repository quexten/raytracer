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
        Vec3::new(10.0, 10.0, 10.0),
        6.0,
        Material::Light(Vec3::new(1.0, 0.6, 0.1).multiply(50.0)),
    )));
    
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.3, -1.5),
        0.3,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.1,
            diffuse: false,
        }),
    )));

    // seeded rand
    let mut seed = [0u8; 32];
    seed[0] = 0x20;
    let mut rng = rand_chacha::ChaCha8Rng::from_seed(seed);
    for i in 0..150 {
        // metal or glow
        let material = Material::Metallic(material::Metallic {
                albedo: Vec3::new(0.5, 0.5, 0.5).add(
                    &hsv_to_rgb(
                        i as f32 * 20.0 % 360.0,
                        1.0,
                        1.0,
                    )
                    .multiply(0.5),
                ),
                fuzz: 0.1,
                diffuse: false,
            });
        let z = (i as f32 * 0.1).cos() - 2.0;
        let x = (i as f32 * 0.1).sin();
        let y = (i as f32) * 0.01 - 0.5;

        world.add(Box::new(sphere::Sphere::new(
            Vec3::new(x, y, z),
            0.1,
            material,
        )));
    }

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
            Box::new(Material::Metallic(material::Metallic {
                albedo: Vec3::new(1.0, 1.0, 1.0),
                fuzz: 0.05,
                diffuse: false,
            })),
            Box::new(Material::Metallic(material::Metallic {
                albedo: Vec3::new(0.2, 0.2, 0.2),
                fuzz: 0.01,
                diffuse: false,
            })),
            5.0,
        ),
    );

    (world, 1.3)
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