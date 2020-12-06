mod renderer;
mod window;
pub mod storage;
mod model;

pub use renderer::{Camera, Renderer};
pub use window::{Gui, get_gui};
pub use model::{RawModel, Triangle};

use na::{Point3, Matrix4};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl Pixel {
    pub const WHITE: Pixel = Pixel { blue: u8::max_value(), green: u8::max_value(), red: u8::max_value() };
    pub const BLACK: Pixel = Pixel { blue: 0, green: 0, red: 0 };
    pub fn new(blue: u8, green: u8, red: u8) -> Self {
        Pixel { blue, green, red }
    }
}

pub fn trilinear_interpolation(tx: f32, ty: f32, tz: f32, c0: &Point3<f32>, c1: &Point3<f32>) -> Point3<f32> {
    let x = unsafe {
        c0.get_unchecked(0) * (1.0 - tx) + c1.get_unchecked(0) * tx
    };
    let y = unsafe {
        c0.get_unchecked(1) * (1.0 - ty) + c1.get_unchecked(1) * ty
    };
    let z = unsafe {
        c0.get_unchecked(2) * (1.0 - tz) + c1.get_unchecked(2) * tz
    };
    Point3::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::{Vector3, IsometryMatrix3};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_trilinear_interpolation() {
        let point1 = Point3::<f32>::new(0.0, 0.0, 0.0);
        let point2 = Point3::<f32>::new(2.0, 2.0, 0.0);
        let res = trilinear_interpolation(0.3, 0.5, 0.0, &point1, &point2);
        assert_approx_eq!(res[0], 0.6, 1e-5);
        assert_approx_eq!(res[1], 1.0, 1e-5);
        assert_approx_eq!(res[2], 0.0, 1e-5);
    }
}
