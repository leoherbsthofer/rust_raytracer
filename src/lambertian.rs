use cgmath::Vector3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::render_engine_multithread::RenderEngineMultithread;

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vector3<f64>,
    emission: f64,
}

impl Material for Lambertian {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f64>, scattered: &mut Ray) -> bool {
        let target = rec.position + rec.normal + RenderEngineMultithread::random_in_unit_sphere();
        /* *scattered = Ray {
            origin: rec.position,
            direction: target - rec.position,
        };*/
        *scattered = Ray::newTime(rec.position, target - rec.position, r_in.time);
        *attenuation = self.albedo;
        //*emissive = false;
        /*if self.emission > 0.1 {
            *emissive = true;
            *emission = self.albedo * self.emission;
        }*/
        return true;
        /*
         * TODO Implement probability p of reflection
         * albedo/p
         */
    }

    fn duplicate(&self) -> Box<dyn Material> {
        return Box::from(Lambertian {
            albedo: self.albedo,
            emission: self.emission,
        });
    }
}

impl Lambertian {
    pub fn new (albedo: Vector3<f64>, emission: f64) -> Lambertian {
        return Lambertian {
            albedo,
            emission,
        }
    }
}