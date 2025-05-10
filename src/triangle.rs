use crate::{hitable::{HitRecord, Hitable}, material::Material, ray::{self, Ray}, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    material: Material,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Material) -> Self {
        Triangle { a, b, c, material }
    }

    pub fn normal(&self) -> Vec3 {
        let e1 = self.b.sub(&self.a);
        let e2 = self.c.sub(&self.a);
        e1.cross(&e2).normalize()
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let intersection = moller_trumbore_intersection(&ray.origin, &ray.direction, self);
        if let Some(intersection_point) = intersection {
            let normal = self.normal();
            let t = intersection_point.sub(&ray.origin).length() / ray.direction.length();

            // flip normal if dot product is negative
            let dot_product = normal.dot(&ray.direction);
            let normal = if dot_product < 0.0 {
                normal
            } else {
                normal.multiply(-1.0)
            };

            if t < t_max && t > t_min {
                return Some(HitRecord {
                    point: intersection_point,
                    normal,
                    t,
                    front_face: true,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}

fn moller_trumbore_intersection (origin: &Vec3, direction: &Vec3, triangle: &Triangle) -> Option<Vec3> {
	let e1 = triangle.b.sub(&triangle.a);
	let e2 = triangle.c.sub(&triangle.a);

	let ray_cross_e2 = direction.cross(&e2);
	let det = e1.dot(&ray_cross_e2);

	if det > -f32::EPSILON && det < f32::EPSILON {
		return None; // This ray is parallel to this triangle.
	}

	let inv_det = 1.0 / det;
	let s = origin.sub(&triangle.a);
	let u = inv_det * s.dot(&ray_cross_e2);
	if u < 0.0 || u > 1.0 {
		return None;
	}

	let s_cross_e1 = s.cross(&e1);
	let v = inv_det * direction.dot(&s_cross_e1);
	if v < 0.0 || u + v > 1.0 {
		return None;
	}
	// At this stage we can compute t to find out where the intersection point is on the line.
	let t = inv_det * e2.dot(&s_cross_e1);

	if t > f32::EPSILON { // ray intersection
		let intersection_point = origin.add(&direction.multiply(t));
		return Some(intersection_point);
	}
	else { // This means that there is a line intersection but not a ray intersection.
		return None;
	}
}