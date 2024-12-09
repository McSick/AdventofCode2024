use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();
    let (mut list1, mut list2) = parse_input(&input);
    let sol1: i64 = sum_diff(&mut list1, &mut list2);
    let sol2: i64 = simularity_score(&mut list1, &mut list2);

    (Solution::from(sol1), Solution::from(sol2))
}

fn sum_diff(list1: &mut Vec<i64>, list2: &mut Vec<i64>) -> i64 {
    list1.sort();
    list2.sort();
    let mut sol: i64 = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        sol += (a - b).abs();
    }
    sol
}

fn simularity_score(list1: &mut Vec<i64>, list2: &mut Vec<i64>) -> i64 {
    let mut list2_map_count = HashMap::new();
    for &mut b in list2 {
        *list2_map_count.entry(b).or_insert(0) += 1;
    }
    let mut sol: i64 = 0;
    for &mut a in list1 {
        if let Some(&count) = list2_map_count.get(&a) {
            sol += a * count;
        }
    }
    sol
}

pub fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        println!("{} {}", a, b);
        list1.push(a.parse().unwrap());
        list2.push(b.parse().unwrap());
    }

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("test/test01.txt").unwrap();
        let (list1, list2) = parse_input(&input);
        assert_eq!(list1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_sum_diff() {
        let input = read_to_string("test/test01.txt").unwrap();
        let (mut list1, mut list2) = parse_input(&input);
        assert_eq!(sum_diff(&mut list1, &mut list2), 11);
    }

    #[test]
    fn test_simularity_score() {
        let input = read_to_string("test/test01.txt").unwrap();
        let (mut list1, mut list2) = parse_input(&input);
        assert_eq!(simularity_score(&mut list1, &mut list2), 31);
    }
}
