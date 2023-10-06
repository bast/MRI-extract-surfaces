use std::collections::{HashMap, HashSet};

use crate::intersection;
use crate::triangle::Triplet;
use crate::vector::Vector3;

type TileIndexMap = HashMap<(isize, isize), HashSet<usize>>;
type TileTripletMap = HashMap<(isize, isize), HashSet<Triplet>>;

pub fn find_inside_points(
    coordinates: &[Vector3],
    tiles_to_points: &TileIndexMap,
    tiles_to_triangles: &TileTripletMap,
    ray_direction: &Vector3,
    ray_direction_opposite: &Vector3,
) -> HashSet<usize> {
    let mut inside_points = HashSet::new();

    for &(ix, iz) in tiles_to_points.keys() {
        let triangles = &tiles_to_triangles[&(ix, iz)];
        for point_index in &tiles_to_points[&(ix, iz)] {
            let x = coordinates[*point_index].x;
            let y = coordinates[*point_index].y;
            let z = coordinates[*point_index].z;

            if intersection::ray_intersects_batch(
                &Vector3 { x, y, z },
                ray_direction,
                coordinates,
                triangles,
            ) && intersection::ray_intersects_batch(
                &Vector3 { x, y, z },
                ray_direction_opposite,
                coordinates,
                triangles,
            ) {
                inside_points.insert(*point_index);
            }
        }
    }

    inside_points
}

pub fn get_step_sizes(num_steps: usize, coordinates: &[Vector3]) -> Vector3 {
    let large_number = f64::MAX;

    let mut x_min = large_number;
    let mut x_max = -large_number;
    let mut y_min = large_number;
    let mut y_max = -large_number;
    let mut z_min = large_number;
    let mut z_max = -large_number;

    for point in coordinates {
        x_min = x_min.min(point.x);
        x_max = x_max.max(point.x);
        y_min = y_min.min(point.y);
        y_max = y_max.max(point.y);
        z_min = z_min.min(point.z);
        z_max = z_max.max(point.z);
    }

    let step_x = (x_max - x_min) / num_steps as f64;
    let step_y = (y_max - y_min) / num_steps as f64;
    let step_z = (z_max - z_min) / num_steps as f64;

    Vector3 {
        x: step_x,
        y: step_y,
        z: step_z,
    }
}

pub fn distribute_points_to_tiles(
    coordinates: &[Vector3],
    step: Vector3,
) -> (TileIndexMap, TileIndexMap) {
    let mut map_along_x = HashMap::new();
    let mut map_along_y = HashMap::new();

    for (i, point) in coordinates.iter().enumerate() {
        let ix = tile_index(point.x, 0.0, step.x);
        let iy = tile_index(point.y, 0.0, step.y);
        let iz = tile_index(point.z, 0.0, step.z);

        map_along_x
            .entry((iy, iz))
            .or_insert(HashSet::new())
            .insert(i);
        map_along_y
            .entry((ix, iz))
            .or_insert(HashSet::new())
            .insert(i);
    }

    (map_along_x, map_along_y)
}

pub fn distribute_triangles_to_tiles(
    coordinates: &[Vector3],
    triangles: &HashSet<Triplet>,
    step: Vector3,
) -> (TileTripletMap, TileTripletMap) {
    let mut map_along_x = HashMap::new();
    let mut map_along_y = HashMap::new();

    for (a, b, c) in triangles {
        let ax = coordinates[*a].x;
        let ay = coordinates[*a].y;
        let az = coordinates[*a].z;
        let bx = coordinates[*b].x;
        let by = coordinates[*b].y;
        let bz = coordinates[*b].z;
        let cx = coordinates[*c].x;
        let cy = coordinates[*c].y;
        let cz = coordinates[*c].z;

        let triangle_x_min = ax.min(bx).min(cx);
        let triangle_x_max = ax.max(bx).max(cx);
        let triangle_y_min = ay.min(by).min(cy);
        let triangle_y_max = ay.max(by).max(cy);
        let triangle_z_min = az.min(bz).min(cz);
        let triangle_z_max = az.max(bz).max(cz);

        let ix_min = tile_index(triangle_x_min, 0.0, step.x);
        let ix_max = tile_index(triangle_x_max, 0.0, step.x);
        let iy_min = tile_index(triangle_y_min, 0.0, step.y);
        let iy_max = tile_index(triangle_y_max, 0.0, step.y);
        let iz_min = tile_index(triangle_z_min, 0.0, step.z);
        let iz_max = tile_index(triangle_z_max, 0.0, step.z);

        for iy in iy_min..=iy_max {
            for iz in iz_min..=iz_max {
                map_along_x
                    .entry((iy, iz))
                    .or_insert(HashSet::new())
                    .insert((*a, *b, *c));
            }
        }

        for ix in ix_min..=ix_max {
            for iz in iz_min..=iz_max {
                map_along_y
                    .entry((ix, iz))
                    .or_insert(HashSet::new())
                    .insert((*a, *b, *c));
            }
        }
    }

    (map_along_x, map_along_y)
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
