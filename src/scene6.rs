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
        Vec3::new(-0.05, 0.02, -1.39),
        0.02,
        Material::Diffuse(Vec3::new(0.2, 0.2, 0.3),
    ))));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(00.05, 0.02, -1.39),
        0.02,
        Material::Diffuse(Vec3::new(0.2, 0.2, 0.3),
    ))));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-0.5, 1.0, -1.5),
        0.2,
        Material::Light(
            Vec3::new(1.0, 0.5, 0.3).multiply(20.0),
        ),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.5, 1.0, -1.5),
        0.2,
        Material::Light(
            Vec3::new(0.2, 0.5, 1.0).multiply(20.0),
        ),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.5),
        0.13,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.8,
            diffuse: true,
        }),
    )));
    
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -0.15, -1.5),
        0.15,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.8,
            diffuse: true,
        }),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -0.4, -1.5),
        0.2,
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.8,
            diffuse: true,
        }),
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
            Box::new(Material::Diffuse(Vec3::new(0.2, 0.2, 0.3))),
            5.0,
        ),
    );

    // mirror left wall
    add_parallelogram(
        &mut world,
        Vec3::new(-1.0, -0.5, -2.0),
        Vec3::new(-1.0, -0.5, 2.0),
        Vec3::new(-1.0, 3.0, 2.0),
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
            diffuse: false,
        }),
    );

    // left wall
    add_parallelogram(
        &mut world,
        Vec3::new(1.0, -0.5, -2.0),
        Vec3::new(1.0, -0.5, 2.0),
        Vec3::new(1.0, 3.0, 2.0),
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
            diffuse: false,
        }),
    );

    // back wall
    add_parallelogram(
        &mut world,
        Vec3::new(-1.0, -0.5, -2.0),
        Vec3::new(1.0, -0.5, -2.0),
        Vec3::new(1.0, 3.0, -2.0),
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
            diffuse: false,
        }),
    );

    add_parallelogram(
        &mut world,
        Vec3::new(-1.0, -0.5, 0.0),
        Vec3::new(1.0, -0.5, 0.0),
        Vec3::new(1.0, 3.0, 0.0),
        Material::Metallic(material::Metallic {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
            diffuse: false,
        }),
    );

            

    (world, 2.4)
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