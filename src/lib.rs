mod renderer;
mod window;

pub use window::{Gui, get_gui};

use std::mem::MaybeUninit;
use std::mem;
use ndarray::{Array3, Array2, arr2, Array1};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

trait Calculator {
    fn scale(&mut self, points: &mut [f32], factor: &[f32]);
    //fn rotate_3(&mut self, points: &mut [f32], factor: &[])
}

struct CPUCalculator;

impl Calculator for CPUCalculator {
    fn scale(&mut self, points: &mut [f32], factor: &[f32]) {
        assert_eq!(points.len() % factor.len(), 0);

        let factor_vec = factor.repeat(points.len() / factor.len() + 1);

        for i in 0..points.len() {
            points[i] *= factor_vec[i];
        }
    }
/*
    fn rotate_3(&mut self, points: &mut [f32], factor: &[_]) {
        unimplemented!()
    }*/
}

fn identity() -> Array2<f32> {
    arr2(&[[1.0,0.0,0.0], [0.0,1.0,0.0], [0.0,0.0,1.0]])
}

fn add_scaling_factor(factor: &[f32], matrix: &mut Array2<f32>) {
    assert_eq!(factor.len(), 3);
    for i in 0..3 {
        matrix[(i, i)] *= factor[i];
    }
}

fn add_rotating_factor(factor: &[f32], matrix: &mut Array2<f32>) {
    assert_eq!(factor.len(), 3);

    let cos_x = factor[0].cos();
    let sin_x = factor[0].sin();
    matrix[(1,1)] += cos_x;
    matrix[(2,2)] += cos_x;
    matrix[(2,1)] += sin_x;
    matrix[(1,2)] += -sin_x;

    let cos_y = factor[1].cos();
    let sin_y = factor[1].sin();
    matrix[(0,0)] += cos_y;
    matrix[(3,3)] += cos_y;
    matrix[(3,0)] += sin_y;
    matrix[(0,3)] += -sin_y;

    let cos_z = factor[2].cos();
    let sin_z = factor[2].sin();
    matrix[(0,0)] += cos_z;
    matrix[(1,1)] += cos_z;
    matrix[(0,1)] += sin_z;
    matrix[(1,0)] += -sin_z;
}

fn calc_normal(l: &Array1<f32>, r: &Array1<f32>) -> Array1<f32> {
    assert_eq!(l.len(), r.len());

    l.iter().zip(r.iter()).map(|(l, r)|(l - r).abs()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() {
        let mut factor = identity();
        add_scaling_factor(&[1.0, 0.0, 1.0], &mut factor);
        assert_eq!(factor, arr2(&[[1.0,0.0,0.0], [0.0,0.0,0.0], [0.0,0.0,1.0]]));
    }
}
