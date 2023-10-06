#![allow(clippy::type_complexity)]

use anyhow::{Context, Result};
use std::collections::HashSet;

use std::fs;
use std::io::{BufWriter, Write};

use crate::vector::Vector3;

pub fn read_mesh(file_name: &str) -> Result<(Vec<Vector3>, HashSet<(usize, usize, usize)>)> {
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).context(error_message.to_string())?;
    let mut lines = contents.lines();

    let mut points = Vec::new();
    let line = lines.next().context(error_message.to_string())?;
    let n: usize = line.parse().context(error_message.to_string())?;

    for _ in 0..n {
        let line = lines.next().context(error_message.to_string())?;
        let words: Vec<&str> = line.split_whitespace().collect();
        ensure!(words.len() == 3, error_message);
        let x: f64 = words[0].parse().context(error_message.to_string())?;
        let y: f64 = words[1].parse().context(error_message.to_string())?;
        let z: f64 = words[2].parse().context(error_message.to_string())?;
        points.push(Vector3 { x, y, z });
    }

    let mut triangles = HashSet::new();
    let line = lines.next().context(error_message.to_string())?;
    let n: usize = line.parse().context(error_message.to_string())?;
    for _ in 0..n {
        let line = lines.next().context(error_message.to_string())?;
        let words: Vec<&str> = line.split_whitespace().collect();
        ensure!(words.len() == 3, error_message);
        let i: usize = words[0].parse().context(error_message.to_string())?;
        let j: usize = words[1].parse().context(error_message.to_string())?;
        let k: usize = words[2].parse().context(error_message.to_string())?;
        triangles.insert((i, j, k));
    }

    Ok((points, triangles))
}

pub fn write_mesh(
    file_name: &str,
    coordinates: &Vec<Vector3>,
    triangles: &HashSet<(usize, usize, usize)>,
) {
    let mut f = BufWriter::new(fs::File::create(file_name).expect("unable to create file"));

    // write points
    writeln!(f, "{}", coordinates.len()).expect("unable to write data");
    for point in coordinates {
        writeln!(f, "{} {} {}", point.x, point.y, point.z).expect("unable to write data");
    }

    // write triangles
    writeln!(f, "{}", triangles.len()).expect("unable to write data");
    for (i, j, k) in triangles {
        writeln!(f, "{} {} {}", i, j, k).expect("unable to write data");
    }
}
