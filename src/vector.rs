pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Triangle {
    pub vertex1: Vector3,
    pub vertex2: Vector3,
    pub vertex3: Vector3,
}

pub fn vec_cross_vec(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn vec_minus_vec(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

pub fn vec_dot_vec(v1: &Vector3, v2: &Vector3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}
