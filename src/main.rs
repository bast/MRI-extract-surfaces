#![allow(clippy::type_complexity)]

use std::collections::{HashMap, HashSet};

mod io;

#[macro_use]
extern crate anyhow;

struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Triangle {
    vertex1: Vector3,
    vertex2: Vector3,
    vertex3: Vector3,
}

fn main() {
    let (coordinates, triangles) = io::read_mesh("data.txt").unwrap();

    let num_steps = 100;

    let ray_direction = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let ray_direction_opposite = Vector3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };

    let (step_x, step_z) = get_step_sizes(num_steps, &coordinates);
    let tiles_to_points = distribute_points_to_tiles(&coordinates, step_x, step_z);
    let tiles_to_triangles =
        distribute_triangles_to_tiles(&coordinates, &triangles, step_x, step_z);

    let mut inside_points = HashSet::new();
    for &(ix, iz) in tiles_to_points.keys() {
        let triangles = &tiles_to_triangles[&(ix, iz)];
        for point_index in &tiles_to_points[&(ix, iz)] {
            let x = coordinates[*point_index].0;
            let y = coordinates[*point_index].1;
            let z = coordinates[*point_index].2;

            if ray_intersects_batch(
                &Vector3 { x, y, z },
                &ray_direction,
                &coordinates,
                triangles,
            ) && ray_intersects_batch(
                &Vector3 { x, y, z },
                &ray_direction_opposite,
                &coordinates,
                triangles,
            ) {
                inside_points.insert(*point_index);
            }
        }
    }

    let mut outside_triangles = HashSet::new();
    for (a, b, c) in &triangles {
        if !inside_points.contains(a) && !inside_points.contains(b) && !inside_points.contains(c) {
            outside_triangles.insert((*a, *b, *c));
        }
    }

    // FIXME: can be compacted further by removing unused points
    io::write_mesh("smaller-data.txt", &coordinates, &outside_triangles);
}

fn ray_intersects_batch(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    coordinates: &[(f64, f64, f64)],
    triangles: &HashSet<(usize, usize, usize)>,
) -> bool {
    for (a, b, c) in triangles {
        let triangle = Triangle {
            vertex1: Vector3 {
                x: coordinates[*a].0,
                y: coordinates[*a].1,
                z: coordinates[*a].2,
            },
            vertex2: Vector3 {
                x: coordinates[*b].0,
                y: coordinates[*b].1,
                z: coordinates[*b].2,
            },
            vertex3: Vector3 {
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

fn get_step_sizes(num_steps: usize, coordinates: &Vec<(f64, f64, f64)>) -> (f64, f64) {
    let large_number = f64::MAX;
    let mut x_min = large_number;
    let mut x_max = -large_number;
    let mut z_min = large_number;
    let mut z_max = -large_number;

    for (x, _, z) in coordinates {
        x_min = x_min.min(*x);
        x_max = x_max.max(*x);
        z_min = z_min.min(*z);
        z_max = z_max.max(*z);
    }

    let step_x = (x_max - x_min) / num_steps as f64;
    let step_z = (z_max - z_min) / num_steps as f64;

    (step_x, step_z)
}

fn distribute_points_to_tiles(
    coordinates: &[(f64, f64, f64)],
    step_x: f64,
    step_z: f64,
) -> HashMap<(isize, isize), HashSet<usize>> {
    let mut mapping = HashMap::new();

    for (i, (x, _, z)) in coordinates.iter().enumerate() {
        let ix = tile_index(*x, 0.0, step_x);
        let iz = tile_index(*z, 0.0, step_z);

        mapping.entry((ix, iz)).or_insert(HashSet::new()).insert(i);
    }

    mapping
}

fn distribute_triangles_to_tiles(
    coordinates: &[(f64, f64, f64)],
    triangles: &HashSet<(usize, usize, usize)>,
    step_x: f64,
    step_z: f64,
) -> HashMap<(isize, isize), HashSet<(usize, usize, usize)>> {
    let mut mapping = HashMap::new();

    for (a, b, c) in triangles {
        let ax = coordinates[*a].0;
        let az = coordinates[*a].2;
        let bx = coordinates[*b].0;
        let bz = coordinates[*b].2;
        let cx = coordinates[*c].0;
        let cz = coordinates[*c].2;

        let triangle_x_min = ax.min(bx).min(cx);
        let triangle_x_max = ax.max(bx).max(cx);
        let triangle_z_min = az.min(bz).min(cz);
        let triangle_z_max = az.max(bz).max(cz);

        let ix_min = tile_index(triangle_x_min, 0.0, step_x);
        let ix_max = tile_index(triangle_x_max, 0.0, step_x);
        let iz_min = tile_index(triangle_z_min, 0.0, step_z);
        let iz_max = tile_index(triangle_z_max, 0.0, step_z);

        for ix in ix_min..=ix_max {
            for iz in iz_min..=iz_max {
                mapping
                    .entry((ix, iz))
                    .or_insert(HashSet::new())
                    .insert((*a, *b, *c));
            }
        }
    }

    mapping
}

fn vec_cross_vec(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

fn vec_minus_vec(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

fn vec_dot_vec(v1: &Vector3, v2: &Vector3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

// written following https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
fn ray_intersects_triangle(
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

fn tile_index(value: f64, origin_value: f64, step: f64) -> isize {
    let d = value - origin_value;
    let r = d / step;

    r.floor() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_index() {
        assert_eq!(tile_index(0.4, 0.0, 1.0), 0);
        assert_eq!(tile_index(-0.4, 0.0, 1.0), -1);
        assert_eq!(tile_index(-0.4, 0.0, 0.4), -1);
        assert_eq!(tile_index(-0.4000001, 0.0, 0.4), -2);
    }
}
