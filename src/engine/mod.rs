use crate::window;
pub mod pixel;

use std::time::{Duration, Instant};
use nalgebra::{Point3, Vector3};

pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}
impl Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let b = 2.0 * (oc.x * ray.direction.x + oc.y * ray.direction.y + oc.z * ray.direction.z);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            if t1 >= 0.0 || t2 >= 0.0 {
                Some(t1.max(t2))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct Cube {
    center: Point3<f64>,
    size: f64,
}
impl Cube {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let half_size = self.size / 2.0;
        let min = self.center - Vector3::new(half_size, half_size, half_size);
        let max = self.center + Vector3::new(half_size, half_size, half_size);

        let t1 = (min.x - ray.origin.x) / ray.direction.x;
        let t2 = (max.x - ray.origin.x) / ray.direction.x;
        let t3 = (min.y - ray.origin.y) / ray.direction.y;
        let t4 = (max.y - ray.origin.y) / ray.direction.y;
        let t5 = (min.z - ray.origin.z) / ray.direction.z;
        let t6 = (max.z - ray.origin.z) / ray.direction.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax >= 0.0 && tmin <= tmax {
            Some(tmin.max(0.0))
        } else {
            None
        }
    }
}

pub fn ProcessScreen(SizeY: usize, SizeX: usize) -> Vec<window::Color> {
    println!("Processing screen (it can take a few seconds)");
    let start = Instant::now();

    let sphere = Sphere {
        center: Point3::new(-1.5, 0.0, -5.0),
        radius: 1.0,
    };
    let cube = Cube {
        center: Point3::new(1.5, 0.0, -5.0),
        size: 1.5,
    };
    let mut result = vec![window::Color::new(0.0, 0.0, 0.0); SizeY*SizeX];
    for i in 0..SizeY*SizeX {
        result[i] = pixel::ProcessPixel(
            SizeY, SizeX,
            i / SizeY,
            i - ((i / SizeY) * SizeY),
            &sphere, &cube,
        )
    }

    println!("Process Screen took {:?}", start.elapsed());
    return result;
}
