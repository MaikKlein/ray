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

    let spheres = vec![
        Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0),
    ];
    let world = World {
        objects: spheres
    };

    let image = cam.render(10, |ray| {
        world.color(ray)
    });
    image.save("test.png").unwrap();
}
