use std::collections::HashSet;

use crate::vector;

pub fn ray_intersects_batch(
    ray_origin: &vector::Vector3,
    ray_direction: &vector::Vector3,
    coordinates: &[(f64, f64, f64)],
    triangles: &HashSet<(usize, usize, usize)>,
) -> bool {
    for (a, b, c) in triangles {
        let triangle = vector::Triangle {
            vertex1: vector::Vector3 {
                x: coordinates[*a].0,
                y: coordinates[*a].1,
                z: coordinates[*a].2,
            },
            vertex2: vector::Vector3 {
                x: coordinates[*b].0,
                y: coordinates[*b].1,
                z: coordinates[*b].2,
            },
            vertex3: vector::Vector3 {
                x: coordinates[*c].0,
                y: coordinates[*c].1,
                z: coordinates[*c].2,
            },
        };
        if ray_intersects_triangle(ray_origin, ray_direction, &triangle) {
            return true;
        }
    }

    false
}

// written following https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
pub fn ray_intersects_triangle(
    ray_origin: &vector::Vector3,
    ray_direction: &vector::Vector3,
    triangle: &vector::Triangle,
) -> bool {
    // let epsilon = f64::EPSILON;
    let epsilon = 0.0000001;

    let edge1 = vector::vec_minus_vec(&triangle.vertex2, &triangle.vertex1);
    let edge2 = vector::vec_minus_vec(&triangle.vertex3, &triangle.vertex1);

    let h = vector::vec_cross_vec(ray_direction, &edge2);
    let a = vector::vec_dot_vec(&edge1, &h);

    if a > -epsilon && a < epsilon {
        // ray is parallel to triangle
        return false;
    }

    let f = 1.0 / a;
    let s = vector::vec_minus_vec(ray_origin, &triangle.vertex1);
    let u = f * vector::vec_dot_vec(&s, &h);

    if u < 0.0 {
        // intersection point is outside triangle
        return false;
    }
    if u > 1.0 {
        // intersection point is outside triangle
        return false;
    }

    let q = vector::vec_cross_vec(&s, &edge1);
    let v = f * vector::vec_dot_vec(ray_direction, &q);

    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * vector::vec_dot_vec(&edge2, &q);

    t > epsilon
}
