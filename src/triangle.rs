use crate::vector::Vector3;

pub struct Triangle {
    pub vertex1: Vector3,
    pub vertex2: Vector3,
    pub vertex3: Vector3,
}

pub type Triplet = (usize, usize, usize);
