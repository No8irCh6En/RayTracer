use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
#[derive(Clone)]
pub struct HitList {
    pub list: Vec<Arc<dyn Hittable>>,
}

impl HitList {
    pub fn new(list: Vec<Arc<dyn Hittable>>) -> Self {
        Self { list }
    }

    pub fn add(&mut self, item: Arc<dyn Hittable>) {
        self.list.push(item);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Hittable for HitList {
    fn hit(&self, ray_: &Ray, int: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, None);
        let mut hit_anything = false;
        let mut closest_so_far = int.max;
        for item in &self.list {
            if item.hit(ray_, Interval::new(int.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = HitRecord {
                    point: temp_rec.point,
                    normal: temp_rec.normal,
                    t: temp_rec.t,
                    front_face: temp_rec.front_face,
                    mat_ptr: temp_rec.mat_ptr.clone(),
                };
            }
        }
        hit_anything
    }
}

unsafe impl Sync for HitList {}
