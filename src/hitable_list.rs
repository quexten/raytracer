use crate::hitable::Hitable;

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl Clone for HitableList {
    fn clone(&self) -> Self {
        let mut new_list = HitableList::new();
        for hitable in &self.list {
            new_list.add(dyn_clone::clone_box(hitable.as_ref()));
        }
        new_list
    }
}

impl HitableList {
    pub fn new() -> Self {
        HitableList { list: Vec::new() }
    }

    pub fn add(&mut self, hitable: Box<dyn Hitable>) {
        self.list.push(hitable);
    }
}

impl Hitable for HitableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hitable::HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<crate::hitable::HitRecord> = None;

        for hitable in &self.list {
            if let Some(record) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}
