use crate::window;
use nalgebra::{Point3, Vector3};
use rand::Rng;

pub fn ProcessPixel(SizeY: usize, SizeX: usize, PosY: usize, PosX: usize, sphere: &super::Sphere, cube: &super::Cube) -> window::Color {
    let aspect_ratio = SizeX as f64 / SizeY as f64;
    let fov_y = 90.0;
    let fov_x = fov_y * aspect_ratio;
    let aspect_ratio_correction_x = (fov_x.to_radians() / 2.0).tan();
    let aspect_ratio_correction_y = (fov_y.to_radians() / 2.0).tan() / aspect_ratio;

    let pixel_x = ((PosX as f64 + 0.5) / SizeX as f64) * 2.0 - 1.0;
    let pixel_y = 1.0 - ((PosY as f64 + 0.5) / SizeY as f64) * 2.0;

    let dir_x = pixel_x * aspect_ratio_correction_x;
    let dir_y = pixel_y * aspect_ratio_correction_y;

    let ray = super::Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        direction: Vector3::new(dir_x, dir_y, -1.0).normalize(),
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

