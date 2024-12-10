use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let map = read_input("input/day10.txt");
    let (sol1, sol2) = find_complete_paths(&map);
    (Solution::from(sol1), Solution::from(sol2))
}
type Map = Vec<Vec<i8>>;
const TRAIL_HEAD: i8 = 0;
const TRAIL_END: i8 = 9;
const MAX_SLOPE: i8 = 1;

fn find_complete_paths(map: &Map) -> (usize, usize) {
    let mut complete_paths = 0;
    let mut ratings = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == TRAIL_HEAD {
                let mut path = HashSet::new();
                dfs(map, (i, j), &mut path);
                let paths = path
                    .into_iter()
                    .filter(|(x, y)| map[*x][*y] == TRAIL_END)
                    .count();
                let rating = visit_cells(map, (i, j));
                complete_paths += paths;
                ratings += rating;
            }
        }
    }
    (complete_paths, ratings)
}

fn dfs(map: &Map, current: (usize, usize), visited: &mut HashSet<(usize, usize)>) {
    visited.insert(current);
    for adjacent in get_adjacent_cells(map, current, visited) {
        dfs(map, adjacent, visited);
    }
}
fn visit_cells(map: &Map, current: (usize, usize)) -> usize {
    if map[current.0][current.1] == TRAIL_END {
        return 1;
    }
    let mut paths = 0;
    for adjacent in get_adjacent_cells(map, current, &mut HashSet::new()) {
        paths += visit_cells(map, adjacent);
    }
    paths
}

const ADJACENT_CELLS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
fn get_adjacent_cells(
    map: &Map,
    current: (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut adjacent_cells = Vec::new();
    for (i, j) in ADJACENT_CELLS {
        if is_within_bounds(map, current, i, j) {
            let next_cell: (usize, usize) = (
                (current.0 as isize + i) as usize,
                (current.1 as isize + j) as usize,
            );
            if !visited.contains(&next_cell)
                && (map[next_cell.0][next_cell.1] - map[current.0][current.1]) == MAX_SLOPE
            {
                adjacent_cells.push(next_cell);
            }
        }
    }
    adjacent_cells
}

fn is_within_bounds(map: &Map, current: (usize, usize), i: isize, j: isize) -> bool {
    !(current.0 == 0 && i == -1
        || current.0 == map.len() - 1 && i == 1
        || current.1 == 0 && j == -1
        || current.1 == map[0].len() - 1 && j == 1)
}

fn read_input(path: &str) -> Map {
    let input = read_to_string(path).unwrap();
    parse_map(&input)
}
fn parse_map(map: &str) -> Map {
    let lines = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();
    lines
}

const SIMPLE_MAP: &str = "0123
1234
8765
9876";

const COMPLEX_MAP: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_complete_paths() {
        let map = parse_map(COMPLEX_MAP);
        assert_eq!(find_complete_paths(&map), (36, 81));
    }
}
