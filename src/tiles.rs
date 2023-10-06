use std::collections::{HashMap, HashSet};

pub fn get_step_sizes(num_steps: usize, coordinates: &Vec<(f64, f64, f64)>) -> (f64, f64) {
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

pub fn distribute_points_to_tiles(
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

pub fn distribute_triangles_to_tiles(
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
