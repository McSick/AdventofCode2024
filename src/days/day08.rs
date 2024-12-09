use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let (points, (width, height)) = read_input("input/day08.txt");
    let (sol1, sol2) = find_unique_antinodes(points, (width, height));

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_unique_antinodes(
    points: HashMap<char, Vec<(f32, f32)>>,
    (width, height): (f32, f32),
) -> (u64, u64) {
    let mut unique_antinodes = HashSet::new();
    let mut unique_antinodes_pt2 = HashSet::new();
    let mut counter = 0;
    for (_, antinodes) in points {
        for point in antinodes.iter().combinations(2) {
            let p1 = *point[0];
            let p2 = *point[1];

            //part 1
            let distance = get_distance(p1, p2);
            let slope = get_slope(p1, p2); //0 is horizontal (X), 1 is vertical (Y)
            let point_left = find_interesting_point(p1, slope, distance, -1.0);
            if is_on_grid(point_left, (width, height)) {
                unique_antinodes.insert((point_left.0 as i32, point_left.1 as i32));
                counter += 1;
            }
            let point_right = find_interesting_point(p2, slope, distance, 1.0);
            if is_on_grid(point_right, (width, height)) {
                unique_antinodes.insert((point_right.0 as i32, point_right.1 as i32));
                counter += 1;
            }

            //part 2
            let antinodes_left = find_all_antinodes_on_line(p1, slope, (width, height), -1.0);
            unique_antinodes_pt2.extend(antinodes_left);
            let antinodes_right = find_all_antinodes_on_line(p2, slope, (width, height), 1.0);
            unique_antinodes_pt2.extend(antinodes_right);
        }
    }
    (
        unique_antinodes.len() as u64,
        unique_antinodes_pt2.len() as u64,
    )
}

fn is_on_grid(point: (f32, f32), (width, height): (f32, f32)) -> bool {
    point.0 >= 0.0 && point.0 < width && point.1 >= 0.0 && point.1 < height
}

// TODO, I am sure i can do math here instead of just moving on the line.
fn find_interesting_point(
    point: (f32, f32),
    slope: (f32, f32),
    distance: f32,
    multiplier: f32,
) -> (f32, f32) {
    let mut current_distance = 0.0;
    let mut new_point = point;
    while current_distance < distance {
        new_point = (
            point.0 + slope.0 * multiplier,
            point.1 + slope.1 * multiplier,
        ); //move on the line
        current_distance = get_distance(point, new_point);
    }
    new_point
}
fn find_all_antinodes_on_line(
    point: (f32, f32),
    slope: (f32, f32),
    (width, height): (f32, f32),
    multiplier: f32,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    antinodes.insert((point.0 as i32, point.1 as i32));
    let mut new_point = point;
    loop {
        new_point = (
            new_point.0 + slope.0 * multiplier,
            new_point.1 + slope.1 * multiplier,
        ); //move on the line
        if is_on_grid(new_point, (width, height)) {
            antinodes.insert((new_point.0 as i32, new_point.1 as i32));
        } else {
            break;
        }
    }
    antinodes
}
fn get_distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt()
}
fn get_slope(p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx, dy)
}

fn read_input(file_name: &str) -> (HashMap<char, Vec<(f32, f32)>>, (f32, f32)) {
    let mut points = HashMap::new();
    let file_string = read_to_string(file_name).unwrap();
    let lines = file_string.lines().collect::<Vec<_>>();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                points.entry(c).or_insert(vec![]).push((x as f32, y as f32));
            }
        }
    }
    (points, (lines.len() as f32, lines[0].len() as f32))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_unique_antinodes() {
        let (points, (width, height)) = read_input("test/test08.txt");
        let (p1result, p2result) = find_unique_antinodes(points, (width, height));
        assert_eq!(p1result, 14);
        assert_eq!(p2result, 34);
    }
}
