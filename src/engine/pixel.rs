use crate::window;
use nalgebra::{Point3, Vector3};
use rand::Rng;

struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

struct Sphere {
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

struct Cube {
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

pub fn ProcessPixel(SizeY: usize, SizeX: usize, PosY: usize, PosX: usize) -> window::Color {
    let aspect_ratio = SizeX as f64 / SizeY as f64;
    let fov_y = 90.0;
    let fov_x = fov_y * aspect_ratio;
    let aspect_ratio_correction_x = (fov_x.to_radians() / 2.0).tan();
    let aspect_ratio_correction_y = (fov_y.to_radians() / 2.0).tan() / aspect_ratio;

    let pixel_x = ((PosX as f64 + 0.5) / SizeX as f64) * 2.0 - 1.0;
    let pixel_y = 1.0 - ((PosY as f64 + 0.5) / SizeY as f64) * 2.0;

    let dir_x = pixel_x * aspect_ratio_correction_x;
    let dir_y = pixel_y * aspect_ratio_correction_y;

    let ray = Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        direction: Vector3::new(dir_x, dir_y, -1.0).normalize(),
    };

    let sphere = Sphere {
        center: Point3::new(-1.5, 0.0, -5.0),
        radius: 1.0,
    };

    let cube = Cube {
        center: Point3::new(1.5, 0.0, -5.0),
        size: 1.5,
    };

    if let Some(distance) = sphere.intersect(&ray) {
        let hit_point = ray.origin + ray.direction * distance;
        let normal = (hit_point - sphere.center).normalize();

        // Calculate color based on the intersection normal
        let color = (normal + Vector3::new(1.0, 1.0, 1.0)) * 0.5;

        return window::Color::new(color.x, color.y, color.z);
    } else if let Some(distance) = cube.intersect(&ray) {
        let hit_point = ray.origin + ray.direction * distance;
        let normal = calculate_cube_normal(hit_point, cube.center, cube.size);

        // Calculate color based on the intersection normal
        let color = (normal + Vector3::new(1.0, 1.0, 1.0)) * 0.5;

        return window::Color::new(color.x, color.y, color.z);
    }

    // If no intersection, return background color
    return window::Color::new(0.0, 0.0, 0.0);
}

fn calculate_cube_normal(point: Point3<f64>, center: Point3<f64>, size: f64) -> Vector3<f64> {
    let half_size = size / 2.0;
    let epsilon = 0.0001;

    let x = if (point.x - (center.x - half_size)).abs() < epsilon {
        -1.0
    } else if (point.x - (center.x + half_size)).abs() < epsilon {
        1.0
    } else {
        0.0
    };

    let y = if (point.y - (center.y - half_size)).abs() < epsilon {
        -1.0
    } else if (point.y - (center.y + half_size)).abs() < epsilon {
        1.0
    } else {
        0.0
    };

    let z = if (point.z - (center.z - half_size)).abs() < epsilon {
        -1.0
    } else if (point.z - (center.z + half_size)).abs() < epsilon {
        1.0
    } else {
        0.0
    };

    Vector3::new(x, y, z)
}

