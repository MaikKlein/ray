extern crate image;
extern crate ray;
use image::Rgb;
use ray::camera::Camera;
use ray::math::*;
fn vec_to_rgb(v: Vector3<f32>) -> Rgb<u8> {
    let b = v.map(|f| {
        assert!(f <= 1.0);
        (f * 255.0) as u8
    });
    Rgb {
        data: [b.x, b.y, b.z],
    }
}
fn main() {
    let cam = Camera::look_at(
        500,
        500,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::unit_z(),
        Vector3::unit_y(),
        10.0,
        Deg(60.0),
    );
    let image = cam.render(|ray| {
        let t = 0.5 * (ray.dir.y + 1.0);
        let v = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        vec_to_rgb(v)
    });

    // Write the contents of this image to the Writer in PNG format.
    image.save("test.png").unwrap();
}
