use clap::Parser;
use std::collections::{HashMap, HashSet};

mod intersection;
mod io;
mod tiles;
mod triangle;
mod vector;

use crate::triangle::Triplet;
use crate::vector::Vector3;

#[macro_use]
extern crate anyhow;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input_file: String,

    /// Output file
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    let (coordinates, triangles) = io::read_mesh(&args.input_file).unwrap();

    let num_steps = 100;
    let step = tiles::get_step_sizes(num_steps, &coordinates);

    let (tiles_to_points_along_x, tiles_to_points_along_y) =
        tiles::distribute_points_to_tiles(&coordinates, step);
    let (tiles_to_triangles_along_x, tiles_to_triangles_along_y) =
        tiles::distribute_triangles_to_tiles(&coordinates, &triangles, step);

    let inside_points_along_x = tiles::find_inside_points(
        &coordinates,
        &tiles_to_points_along_x,
        &tiles_to_triangles_along_x,
        &Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        &Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
    );
    let inside_points_along_y = tiles::find_inside_points(
        &coordinates,
        &tiles_to_points_along_y,
        &tiles_to_triangles_along_y,
        &Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        &Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
    );

    let inside_points: HashSet<_> = inside_points_along_x
        .intersection(&inside_points_along_y)
        .collect();

    let mut outside_triangles = HashSet::new();
    for (a, b, c) in &triangles {
        if !inside_points.contains(a) && !inside_points.contains(b) && !inside_points.contains(c) {
            outside_triangles.insert((*a, *b, *c));
        }
    }

    let (coordinates, triangles) = remove_unused_points(&coordinates, &outside_triangles);
    io::write_mesh(&args.output_file, &coordinates, &triangles);
}

fn remove_unused_points(
    coordinates: &[Vector3],
    triangles: &HashSet<Triplet>,
) -> (Vec<Vector3>, HashSet<Triplet>) {
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

    let mut new_triangles: HashSet<Triplet> = HashSet::new();
    for (a, b, c) in triangles {
        let a_new = *point_index_map.get(a).unwrap();
        let b_new = *point_index_map.get(b).unwrap();
        let c_new = *point_index_map.get(c).unwrap();
        new_triangles.insert((a_new, b_new, c_new));
    }

    (new_points, new_triangles)
}
