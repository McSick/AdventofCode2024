use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;
    let input = read_to_string("input/day02.txt").unwrap();
    let lines = parse_input(&input);
    for line in lines {
        if line.is_safe {
            sol1 += 1;
        }
        if line.is_safe_with_single_removal {
            sol2 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|line| Line::new(line)).collect()
}

#[derive(Debug)]
struct Line {
    levels: Vec<i64>,
    is_safe: bool,
    is_safe_with_single_removal: bool,
}
impl Line {
    fn new(line: &str) -> Self {
        let levels: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            levels: levels.clone(),
            is_safe: is_safe(&levels.clone()),
            is_safe_with_single_removal: is_safe_with_single_removal(&mut levels.clone()),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
    Invalid,
}

const MAX_DIFF: i64 = 3;
fn is_safe(levels: &Vec<i64>) -> bool {
    let mut current_direction: Option<Direction> = None;
    for i in 1..levels.len() {
        let next_direction = get_direction(levels[i - 1], levels[i]);
        match (current_direction, next_direction) {
            (_, Direction::Invalid) => return false,
            (None, _direction) => {}
            (Some(current), next) => {
                if current != next {
                    return false;
                }
            }
        }
        current_direction = Some(next_direction);
    }
    true
}

// Brute force makes me sad
fn is_safe_with_single_removal(levels: &mut Vec<i64>) -> bool {
    if is_safe(&levels) {
        true
    } else {
        for i in 0..levels.len() {
            let mut sub_levels = levels.clone();
            sub_levels.remove(i);
            if is_safe(&sub_levels) {
                return true;
            }
        }
        false
    }
}
fn get_direction(a: i64, b: i64) -> Direction {
    if a > b && (a - b).abs() <= MAX_DIFF {
        Direction::Decreasing
    } else if a < b && (b - a).abs() <= MAX_DIFF {
        Direction::Increasing
    } else {
        Direction::Invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_is_safe_with_single_removal() {
        assert_eq!(is_safe_with_single_removal(&mut vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe_with_single_removal(&mut vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe_with_single_removal(&mut vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe_with_single_removal(&mut vec![1, 3, 2, 4, 5]), true);
        assert_eq!(is_safe_with_single_removal(&mut vec![8, 6, 4, 4, 1]), true);
        assert_eq!(is_safe_with_single_removal(&mut vec![1, 3, 6, 7, 9]), true);
    }
}
