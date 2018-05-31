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
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let image = cam.render(100, |ray| {
        if let Some(rayhit) = sphere.intersect(ray) {
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
