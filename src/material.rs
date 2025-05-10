use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum Material {
    Diffuse(Vec3),
    Light(Vec3),
    Metallic(Metallic),
    CheckerBoard(Box<Material>, Box<Material>, f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Metallic {
    pub albedo: Vec3,
    pub fuzz: f32,
}
