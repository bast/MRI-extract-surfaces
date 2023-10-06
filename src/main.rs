#![allow(clippy::type_complexity)]

use std::collections::HashSet;

mod intersection;
mod io;
mod tiles;
mod vector;

#[macro_use]
extern crate anyhow;

fn main() {
    let (coordinates, triangles) = io::read_mesh("data.txt").unwrap();

    let num_steps = 100;

    let ray_direction = vector::Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let ray_direction_opposite = vector::Vector3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };

    let (step_x, step_z) = tiles::get_step_sizes(num_steps, &coordinates);
    let tiles_to_points = tiles::distribute_points_to_tiles(&coordinates, step_x, step_z);
    let tiles_to_triangles =
        tiles::distribute_triangles_to_tiles(&coordinates, &triangles, step_x, step_z);

    let mut inside_points = HashSet::new();
    for &(ix, iz) in tiles_to_points.keys() {
        let triangles = &tiles_to_triangles[&(ix, iz)];
        for point_index in &tiles_to_points[&(ix, iz)] {
            let x = coordinates[*point_index].0;
            let y = coordinates[*point_index].1;
            let z = coordinates[*point_index].2;

            if intersection::ray_intersects_batch(
                &vector::Vector3 { x, y, z },
                &ray_direction,
                &coordinates,
                triangles,
            ) && intersection::ray_intersects_batch(
                &vector::Vector3 { x, y, z },
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
