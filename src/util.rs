pub fn random_double() -> f32 {
    let mut rng = rand::rng();
    rand::Rng::random_range(&mut rng, 0.0..1.0)
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    min + random_double() * (max - min)
}
