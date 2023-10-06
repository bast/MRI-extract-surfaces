#![allow(clippy::type_complexity)]

use std::collections::{HashMap, HashSet};

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

    let step = tiles::get_step_sizes(num_steps, &coordinates);
    let tiles_to_points = tiles::distribute_points_to_tiles(&coordinates, step.x, step.z);
    let tiles_to_triangles =
        tiles::distribute_triangles_to_tiles(&coordinates, &triangles, step.x, step.z);

    let mut inside_points = HashSet::new();
    for &(ix, iz) in tiles_to_points.keys() {
        let triangles = &tiles_to_triangles[&(ix, iz)];
        for point_index in &tiles_to_points[&(ix, iz)] {
            let x = coordinates[*point_index].x;
            let y = coordinates[*point_index].y;
            let z = coordinates[*point_index].z;

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

    let (coordinates, triangles) = remove_unused_points(&coordinates, &outside_triangles);
    io::write_mesh("smaller-data.txt", &coordinates, &triangles);
}

fn remove_unused_points(
    coordinates: &[vector::Vector3],
    triangles: &HashSet<(usize, usize, usize)>,
) -> (Vec<vector::Vector3>, HashSet<(usize, usize, usize)>) {
    let used_indices: HashSet<usize> = triangles
        .iter()
        .flat_map(|&(a, b, c)| vec![a, b, c])
        .collect();

    let mut used_indices: Vec<usize> = used_indices.into_iter().collect();
    used_indices.sort_unstable();

    let mut new_points = Vec::new();
    let mut point_index_map: HashMap<usize, usize> = HashMap::new();
    for (i, j) in used_indices.iter().enumerate() {
        point_index_map.insert(*j, i);
        new_points.push(coordinates[*j]);
    }

    let mut new_triangles: HashSet<(usize, usize, usize)> = HashSet::new();
    for (a, b, c) in triangles {
        let a_new = *point_index_map.get(a).unwrap();
        let b_new = *point_index_map.get(b).unwrap();
        let c_new = *point_index_map.get(c).unwrap();
        new_triangles.insert((a_new, b_new, c_new));
    }

    (new_points, new_triangles)
}
