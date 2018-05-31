use cgmath::{InnerSpace, Vector3};

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
}
impl Camera {
    pub fn look_at(
        width: u32,
        height: u32,
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        up: Vector3<f32>,
        focus_dist: f32,
    ) -> Camera {
        let w = InnerSpace::normalize(look_at - look_from);
        let u = InnerSpace::normalize(up.cross(w));
        let v = w.cross(u);
        Camera {
            origin: look_from,
            w,
            u,
            v,
            focus_dist,
            width,
            height
        }
    }
    pub fn shoot_rays<F>(&self, f: F)
    where
        F: Fn(),
    {

    }
}
