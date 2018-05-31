use cgmath::{Deg, InnerSpace, Rad, Vector3};
use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use math::Ray;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub origin: Vector3<f32>,
    // view vector
    pub w: Vector3<f32>,
    // right vector
    pub u: Vector3<f32>,
    // up vector
    pub v: Vector3<f32>,
    pub focus_dist: f32,
    pub fov: Deg<f32>,
}
impl Camera {
    pub fn look_at(
        width: u32,
        height: u32,
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        up: Vector3<f32>,
        focus_dist: f32,
        fov: Deg<f32>,
    ) -> Camera {
        let w = InnerSpace::normalize(look_at - look_from);
        let u = InnerSpace::normalize(up.cross(w));
        let v = w.cross(u);
        Camera {
            fov,
            origin: look_from,
            w,
            u,
            v,
            focus_dist,
            width,
            height,
        }
    }
    pub fn render<F>(&self, f: F) -> RgbImage
    where
        F: Fn(Ray) -> Rgb<u8>,
    {
        let mut image_buffer = ImageBuffer::new(self.width, self.height);
        let half_fov = Rad::from(self.fov).0;
        let right = self.u * self.focus_dist * f32::tan(half_fov);
        let up = self.v * self.focus_dist * f32::tan(half_fov);

        let forward = self.w * self.focus_dist;
        let lower_left_corner = forward - right - up;
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let u = x as f32 / self.width as f32;
                let v = y as f32 / self.height as f32;
                let dir = lower_left_corner + (right * 2.0 * u) + (up * 2.0 * v);
                let ray = Ray::new(self.origin, dir.normalize());
                let rgb = f(ray);
                image_buffer.put_pixel(x, y, rgb);
            }
        }
        image_buffer
    }
}
