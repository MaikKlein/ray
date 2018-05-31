extern crate image;
extern crate ray;
use image::Rgb;
use ray::camera::Camera;
use ray::math::*;
use ray::primitive::Sphere;
fn main() {
    let cam = Camera::look_at(
        800,
        800,
        Vector3::new(0.0, 0.0, 0.0),
        -Vector3::unit_z(),
        Vector3::unit_y(),
        10.0,
        Deg(60.0),
    );

    let spheres = [
        Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let image = cam.render(100, |ray| {
        let hit = spheres.iter().filter_map(|s| s.intersect(ray)).nth(0);
        if let Some(rayhit) = hit {
            let normal = rayhit.normal;
            let normal = 0.5 * (normal + Vector3::new(1.0, 1.0, 1.0));
            normal
        } else {
            let t = 0.5 * (ray.dir.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    });
    image.save("test.png").unwrap();
}
