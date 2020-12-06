use crate::storage::{Point3ID, VecStorage, Storage};
use na::{Vector3, Point3};

#[derive(Debug, PartialEq)]
pub struct Triangle {
    points: [Point3ID; 3]
}

impl Triangle {
    pub fn new(points: [Point3ID; 3]) -> Self {
        Triangle { points }
    }
}

#[derive(Debug)]
pub struct Ray<'a> {
    start_position: &'a Point3<f32>,
    vector: &'a Vector3<f32>,
}

impl<'a> Ray<'a> {
    pub fn new(start_position: &'a Point3<f32>, vector: &'a Vector3<f32>) -> Self {
        Ray { start_position, vector }
    }
}

#[derive(Debug)]
pub struct RawModel {
    storage: VecStorage,
    triangles: Vec<Triangle>,
    center: Point3<f32>,
    forward: Vector3<f32>,
}

impl RawModel {
    pub fn new(vertices: Vec<Point3<f32>>, triangles: Vec<Triangle>) -> Self {
        let forward = Vector3::z();
        let center = calculate_center(&vertices);
        RawModel { storage: VecStorage::from_vec(vertices), triangles, center, forward }
    }
    pub fn triangles_mut(&mut self) -> &mut [Triangle] {
        &mut self.triangles
    }
    pub fn storage_mut(&mut self) -> &mut VecStorage {
        &mut self.storage
    }
}

pub trait Model {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Model for RawModel {
    fn intersect(&self, ray: &Ray) -> bool {
        for triangle in self.triangles.iter() {
            if intersect_triangle(triangle, &self.storage, ray) {
                return true;
            }
        }
        false
    }
}

fn intersect_triangle<S: Storage<Point3<f32>, Id=Point3ID>>(triangle: &Triangle, storage: &S, ray: &Ray) -> bool {
    let v1 = unsafe { storage.get_unchecked(triangle.points[0]) };
    let v2 = unsafe { storage.get_unchecked(triangle.points[1]) };
    let v3 = unsafe { storage.get_unchecked(triangle.points[2]) };

    let v1v2: Vector3<f32> = v2 - v1;
    let v1v3: Vector3<f32> = v3 - v1;

    let normal: Vector3<f32> = v1v2.cross(&v1v3);

    let normal_dot_ray = normal.dot(&ray.vector);
    if normal_dot_ray < 1e-4 {
        return false;
    }

    let d = normal.dot(&v1.coords);

    let distance_to_plane: f32 = (normal.dot(&ray.start_position.coords) + d) / normal_dot_ray;

    if distance_to_plane < 0.0 { return false; }

    let intersection_point = ray.start_position + distance_to_plane * ray.vector;

    let edge0 = v2 - v1;
    let vector_to_inter_point = intersection_point - v1;
    let c = edge0.cross(&vector_to_inter_point);
    if normal.dot(&c) < 0.0 { return false; }

    let edge1 = v3 - v2;
    let vector_to_inter_point = intersection_point - v2;
    let c = edge1.cross(&vector_to_inter_point);
    if normal.dot(&c) < 0.0 { return false; }

    let edge2 = v1 - v3;
    let vector_to_inter_point = intersection_point - v3;
    let c = edge2.cross(&vector_to_inter_point);
    if normal.dot(&c) < 0.0 { return false; }

    true
}

fn calculate_center(vertices: &[Point3<f32>]) -> Point3<f32> {
    assert!(vertices.len() > 0);
    let mut point = Point3::<f32>::origin();
    for vertice in vertices {
        point += vertice.coords;
    }
    point / (vertices.len() as f32)
}
