use crate::part1::{is_valid, parse_triangles, Triangle};

pub fn count_valid_in_columns(input: &str) -> usize {
    let triangles = parse_triangles(input);
    invert_triangles(&triangles)
        .iter()
        .filter(|&triangle| is_valid(triangle))
        .count()
}

pub fn invert_triangles(triangles: &[Triangle]) -> Vec<Triangle> {
    triangles
        .chunks(3)
        .into_iter()
        .flat_map(|tris| {
            [
                (tris[0].0, tris[1].0, tris[2].0),
                (tris[0].1, tris[1].1, tris[2].1),
                (tris[0].2, tris[1].2, tris[2].2),
            ]
        })
        .collect()
}
