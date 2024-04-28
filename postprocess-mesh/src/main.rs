#![allow(clippy::type_complexity)]

use clap::Parser;
use std::collections::{HashMap, HashSet};

mod io;

#[macro_use]
extern crate anyhow;

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

    let triangles = orient_triangles(&triangles, true);
    let triangles = remove_double_boundary_indices(&triangles);
    let triangles = remove_hourglass_indices(&triangles);

    let (coordinates, triangles) = remove_unreferenced_indices(&coordinates, &triangles);

    io::write_mesh(&args.output_file, &coordinates, &triangles);
}

fn remove_unreferenced_indices(
    coordinates: &[(f64, f64, f64)],
    triangles: &HashSet<(usize, usize, usize)>,
) -> (Vec<(f64, f64, f64)>, HashSet<(usize, usize, usize)>) {
    let used_indices: HashSet<usize> = triangles
        .iter()
        .flat_map(|&(a, b, c)| vec![a, b, c])
        .collect();

    let mut used_indices: Vec<usize> = used_indices.into_iter().collect();
    used_indices.sort_unstable();

    let mut new_coordinates = Vec::new();
    let mut point_index_map: HashMap<usize, usize> = HashMap::new();
    for (i, j) in used_indices.iter().enumerate() {
        point_index_map.insert(*j, i);
        new_coordinates.push(coordinates[*j]);
    }

    let mut new_triangles: HashSet<(usize, usize, usize)> = HashSet::new();
    for (a, b, c) in triangles {
        let a_new = *point_index_map.get(a).unwrap();
        let b_new = *point_index_map.get(b).unwrap();
        let c_new = *point_index_map.get(c).unwrap();
        new_triangles.insert((a_new, b_new, c_new));
    }

    (new_coordinates, new_triangles)
}

fn ordered(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn triangle_neighbors(
    triangles: &HashSet<(usize, usize, usize)>,
) -> HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> {
    let mut edge_to_triangles: HashMap<(usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();
    for (a, b, c) in triangles {
        edge_to_triangles
            .entry(ordered(*a, *b))
            .or_default()
            .push((*a, *b, *c));
        edge_to_triangles
            .entry(ordered(*b, *c))
            .or_default()
            .push((*a, *b, *c));
        edge_to_triangles
            .entry(ordered(*c, *a))
            .or_default()
            .push((*a, *b, *c));
    }

    let mut neighbors: HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();
    for (_edge, triangles) in edge_to_triangles {
        if triangles.len() == 2 {
            let (t1, t2) = (triangles[0], triangles[1]);
            neighbors.entry(t1).or_default().push(t2);
            neighbors.entry(t2).or_default().push(t1);
        }
    }

    neighbors
}

/// Start with start_triangle
/// Then visit all its neighbors before going anywhere else
/// Then visit all neighbors of visited triangles
/// And so on ...
fn visit_all_triangles(
    triangles: &HashSet<(usize, usize, usize)>,
    start_triangle: (usize, usize, usize),
) -> Vec<(usize, usize, usize)> {
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut visit_list: Vec<(usize, usize, usize)> = Vec::new();
    let neighbors = triangle_neighbors(triangles);

    let mut to_visit = std::collections::VecDeque::new();
    to_visit.push_back(start_triangle);
    while let Some(triangle) = to_visit.pop_front() {
        if visited.contains(&triangle) {
            continue;
        }
        visited.insert(triangle);
        visit_list.push(triangle);
        if let Some(neighbors) = neighbors.get(&triangle) {
            for neighbor in neighbors {
                to_visit.push_back(*neighbor);
            }
        }
    }

    visit_list
}

fn orient_triangles(
    triangles: &HashSet<(usize, usize, usize)>,
    drop_bad_triangles: bool,
) -> HashSet<(usize, usize, usize)> {
    let start_triangle = *triangles.iter().next().unwrap();
    let visit_list = visit_all_triangles(triangles, start_triangle);

    let mut half_edges: HashSet<(usize, usize)> = HashSet::new();
    let mut oriented_triangles: HashSet<(usize, usize, usize)> = HashSet::new();
    for (a, b, c) in visit_list {
        if half_edges.contains(&(a, b))
            || half_edges.contains(&(b, c))
            || half_edges.contains(&(c, a))
        {
            if !drop_bad_triangles {
                oriented_triangles.insert((c, b, a));
                half_edges.insert((c, b));
                half_edges.insert((b, a));
                half_edges.insert((a, c));
            }
        } else {
            oriented_triangles.insert((a, b, c));
            half_edges.insert((a, b));
            half_edges.insert((b, c));
            half_edges.insert((c, a));
        }
    }

    oriented_triangles
}

fn remove_double_boundary_indices(
    triangles: &HashSet<(usize, usize, usize)>,
) -> HashSet<(usize, usize, usize)> {
    let mut edge_to_vertex: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (a, b, c) in triangles {
        edge_to_vertex.entry(ordered(*a, *b)).or_default().push(*c);
        edge_to_vertex.entry(ordered(*b, *c)).or_default().push(*a);
        edge_to_vertex.entry(ordered(*c, *a)).or_default().push(*b);
    }

    let mut boundary_indices: HashMap<usize, usize> = HashMap::new();
    for ((a, b), vs) in edge_to_vertex {
        if vs.len() == 1 {
            *boundary_indices.entry(a).or_insert(0) += 1;
            *boundary_indices.entry(b).or_insert(0) += 1;
        }
    }

    let double_boundary_indices: HashSet<usize> = boundary_indices
        .iter()
        .filter(|(_, &v)| v > 2)
        .map(|(&k, _)| k)
        .collect();

    let mut new_triangles: HashSet<(usize, usize, usize)> = HashSet::new();
    for (a, b, c) in triangles {
        if double_boundary_indices.contains(a)
            || double_boundary_indices.contains(b)
            || double_boundary_indices.contains(c)
        {
            continue;
        }
        new_triangles.insert((*a, *b, *c));
    }

    new_triangles
}

fn dfs(node: usize, adj: &HashMap<usize, HashSet<usize>>, visited: &mut HashMap<usize, bool>) {
    let mut stack = vec![node];
    while let Some(vertex) = stack.pop() {
        if !visited[&vertex] {
            visited.insert(vertex, true);
            for neighbor in &adj[&vertex] {
                if !visited[&neighbor] {
                    stack.push(*neighbor);
                }
            }
        }
    }
}

fn number_of_components(edges: &HashSet<(usize, usize)>) -> usize {
    if edges.is_empty() {
        return 0;
    }

    let mut adj: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (u, v) in edges {
        adj.entry(*u).or_default().insert(*v);
        adj.entry(*v).or_default().insert(*u);
    }

    let mut visited: HashMap<usize, bool> = adj.keys().map(|&k| (k, false)).collect();
    let mut component_count = 0;

    for node in adj.keys() {
        if !visited[node] {
            dfs(*node, &adj, &mut visited);
            component_count += 1;
        }
    }

    component_count
}

fn remove_hourglass_indices(
    triangles: &HashSet<(usize, usize, usize)>,
) -> HashSet<(usize, usize, usize)> {
    let mut indices: HashSet<usize> = HashSet::new();
    for (a, b, c) in triangles {
        indices.insert(*a);
        indices.insert(*b);
        indices.insert(*c);
    }

    let mut opposite_edges: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    for (a, b, c) in triangles {
        opposite_edges
            .entry(*a)
            .or_default()
            .insert(ordered(*b, *c));
        opposite_edges
            .entry(*b)
            .or_default()
            .insert(ordered(*c, *a));
        opposite_edges
            .entry(*c)
            .or_default()
            .insert(ordered(*a, *b));
    }

    let mut bad_indices: HashSet<usize> = HashSet::new();
    for index in &indices {
        let components = number_of_components(&opposite_edges[index]);
        if components > 1 {
            bad_indices.insert(*index);
        }
    }

    let mut new_triangles: HashSet<(usize, usize, usize)> = HashSet::new();
    for (a, b, c) in triangles {
        if bad_indices.contains(a) || bad_indices.contains(b) || bad_indices.contains(c) {
            continue;
        }
        new_triangles.insert((*a, *b, *c));
    }

    new_triangles
}
