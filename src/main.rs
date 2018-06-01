extern crate image;
extern crate ray;
use ray::camera::Camera;
use ray::material::{Lambert, Material, Object};
use ray::math::*;
use ray::primitive::{Primitive, Sphere};
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

    let objects = vec![
        
        Object {
            primitive: Primitive::Sphere(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
            material: Material::Lambert(Lambert{ albedo: Vector3::new(1.0, 0.0, 0.0) })
        },
        
        Object {
            primitive: Primitive::Sphere(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
            material: Material::Lambert(Lambert{ albedo: Vector3::new(0.7, 0.7, 0.7) })
        }
    ];
    let world = World { objects };

    let image = cam.render(10, |ray| world.color(ray));
    image.save("test.png").unwrap();
}
