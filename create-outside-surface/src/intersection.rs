use std::collections::HashSet;

use crate::triangle::Triangle;
use crate::vector::{vec_cross_vec, vec_dot_vec, vec_minus_vec, Vector3};

pub fn ray_intersects_batch(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    coordinates: &[Vector3],
    triangles: &HashSet<(usize, usize, usize)>,
) -> bool {
    for (a, b, c) in triangles {
        let triangle = Triangle {
            vertex1: Vector3 {
                x: coordinates[*a].x,
                y: coordinates[*a].y,
                z: coordinates[*a].z,
            },
            vertex2: Vector3 {
                x: coordinates[*b].x,
                y: coordinates[*b].y,
                z: coordinates[*b].z,
            },
            vertex3: Vector3 {
                x: coordinates[*c].x,
                y: coordinates[*c].y,
                z: coordinates[*c].z,
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
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    triangle: &Triangle,
) -> bool {
    // let epsilon = f64::EPSILON;
    let epsilon = 0.0000001;

    let edge1 = vec_minus_vec(&triangle.vertex2, &triangle.vertex1);
    let edge2 = vec_minus_vec(&triangle.vertex3, &triangle.vertex1);

    let h = vec_cross_vec(ray_direction, &edge2);
    let a = vec_dot_vec(&edge1, &h);

    if a > -epsilon && a < epsilon {
        // ray is parallel to triangle
        return false;
    }

    let f = 1.0 / a;
    let s = vec_minus_vec(ray_origin, &triangle.vertex1);
    let u = f * vec_dot_vec(&s, &h);

    if u < 0.0 {
        // intersection point is outside triangle
        return false;
    }
    if u > 1.0 {
        // intersection point is outside triangle
        return false;
    }

    let q = vec_cross_vec(&s, &edge1);
    let v = f * vec_dot_vec(ray_direction, &q);

    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * vec_dot_vec(&edge2, &q);

    t > epsilon
}
