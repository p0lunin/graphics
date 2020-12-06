use na::{Point3};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point3ID(pub usize);

impl From<usize> for Point3ID {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

pub trait Storage<T> {
    type Id;
    fn store(&mut self, element: T) -> Self::Id;
    fn get(&self, id: Self::Id) -> Option<&T>;
    unsafe fn get_unchecked(&self, id: Self::Id) -> &T;
    fn get_mut(&mut self, id: Self::Id) -> Option<&mut T>;
    unsafe fn get_unchecked_mut(&mut self, id: Self::Id) -> &mut T;
    fn update_all(&mut self, f: impl Fn(&mut T));
}

#[derive(Debug)]
pub struct VecStorage {
    points: Vec<Point3<f32>>,
}

impl VecStorage {
    pub fn new() -> Self {
        Self {
            points: vec![]
        }
    }

    pub fn from_vec(points: Vec<Point3<f32>>) -> Self {
        Self {
            points
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(capacity)
        }
    }
}

impl Storage<Point3<f32>> for VecStorage {
    type Id = Point3ID;

    fn store(&mut self, point: Point3<f32>) -> Point3ID {
        self.points.push(point);
        Point3ID(self.points.len() - 1)
    }

    fn get(&self, id: Point3ID) -> Option<&Point3<f32>> {
        if self.points.len() > id.0 {
            Some(unsafe { self.get_unchecked(id) })
        }
        else {
            None
        }
    }

    unsafe fn get_unchecked(&self, id: Point3ID) -> &Point3<f32> {
        self.points.get_unchecked(id.0)
    }

    fn get_mut(&mut self, id: Self::Id) -> Option<&mut Point3<f32>> {
        if self.points.len() > id.0 {
            Some(unsafe { self.get_unchecked_mut(id) })
        }
        else {
            None
        }
    }

    unsafe fn get_unchecked_mut(&mut self, id: Self::Id) -> &mut Point3<f32> {
        self.points.get_unchecked_mut(id.0)
    }

    fn update_all(&mut self, f: impl Fn(&mut Point3<f32>)) {
        self.points.iter_mut().for_each(f);
    }
}
