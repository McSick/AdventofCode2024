use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day11.txt").unwrap();

    let mut map = parse_input(&input);
    let mut map_part2 = map.clone();
    map = apply_rules(&mut map, 25);
    let sol1 = reduce_map(map);

    map_part2 = apply_rules(&mut map_part2, 75);
    let sol2 = reduce_map(map_part2);
    (Solution::from(sol1), Solution::from(sol2))
}

fn reduce_map(map: HashMap<u64, u64>) -> u64 {
    let mut sum = 0;
    for (num, count) in map.iter() {
        sum += *count
    }
   sum
}
fn apply_rules(map: &mut HashMap<u64, u64>, num_iterations: u64) -> HashMap<u64, u64> {
    let mut new_map: HashMap<u64, u64> = HashMap::new();
    for _ in 0..num_iterations {
        new_map.clear();
        for (num, count) in map.iter() {
            let new_nums = rule(*num);
            for new_num in new_nums {
                *new_map.entry(new_num).or_insert(0) += *count;
            }
        }
        map.clear();
        map.extend(new_map.clone());
    }
    new_map
}
fn rule(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }
    let num_str = num.to_string();
    if num_str.len() % 2 == 0 {
        let (left, right) = num_str.split_at(num_str.len() / 2);
        return vec![left.parse().unwrap(), right.parse().unwrap()];
    }
    vec![num * 2024]
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    let numbers = input.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    for (i, &num) in numbers.iter().enumerate() {
        map.insert(num, 1);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rules() {
        let input = "0 1 10 99 999";
        let mut map = parse_input(input);
        let map = apply_rules(&mut map, 1);
        println!("{:?}", map);
        assert_eq!(reduce_map(map), 7);
        let input = "125 17";
        let mut map = parse_input(input);
        let map = apply_rules(&mut map, 6);
        println!("{:?}", map);
        assert_eq!(reduce_map(map), 22);
        
    }
}
