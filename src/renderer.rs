use crate::Pixel;
use crate::model::{Model, Ray};
use na::{Point3, Vector3, Isometry3, Matrix4};
use std::time::Instant;

#[derive(Debug)]
pub struct Camera {
    position: Point3<f32>,
    to: Point3<f32>,
}

impl Camera {
    pub fn new(position: Point3<f32>, to: Point3<f32>) -> Self {
        Camera { position, to }
    }
}

pub struct Renderer {
    canvas: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: vec![Pixel::BLACK; width * height],
            width,
            height,
        }
    }
    pub fn canvas(&self) -> &[Pixel] {
        &self.canvas
    }
    pub fn render<M: Model>(&mut self, camera: &Camera, models: &[M]) {
        let camera_direction = camera.to - camera.position;
        let forward = (-camera_direction).normalize();
        let right = Vector3::new(0.0, 1.0, 0.0).normalize().cross(&forward);
        let up = forward.cross(&right);
        let translation = Matrix4::look_at_rh(
            &camera.position,
            &camera.to,
            &up,
        );
        let width_f = self.width as f32;
        let height_f = self.height as f32;

        let transform_left_top = translation.transform_vector(&Vector3::new(1.0, 1.0, 0.0));

        let appendix_x = ((1.0 - 1.0 / width_f) * 2.0) - 1.0;
        let appendix_y = ((1.0 - 1.0 / height_f) * 2.0) - 1.0;
        let delta_x = translation.transform_vector(&Vector3::new(appendix_x, 1.0, 0.0))
            - transform_left_top;
        let delta_y = translation.transform_vector(&Vector3::new(1.0, appendix_y, 0.0))
            - transform_left_top;
        let mut ray_direction = camera_direction + transform_left_top;
        let ray_direction_delta_x_512 = delta_x * 512.0;

        for y in 0..self.height {
            for x in 0..self.width {
                ray_direction += delta_x;
                let ray = Ray::new(&camera.position, &ray_direction);

                for m in models {
                    if m.intersect(&ray) {
                        self.canvas[y*self.width+x] = Pixel::WHITE;
                    }
                    else {
                        self.canvas[y*self.width+x] = Pixel::BLACK;
                    }
                }
            }
            ray_direction -= ray_direction_delta_x_512;
            ray_direction += delta_y;
        }
    }

}
