use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let (map_order, updates) = read_input("input/day05.txt");
    let (sol1, sol2) = sum_middles(&map_order, &updates);

    (Solution::from(sol1), Solution::from(sol2))
}

fn read_input(filename: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let input = read_to_string(filename).unwrap();
    let mut map_order: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut read_map = true;

    let mut updates: Vec<Vec<u64>> = Vec::new();
    let (rules_string, updates_string) = input.split_once("\n\n").unwrap();
    for rule in rules_string.lines() {
        let (key, value) = rule.trim().split_once("|").unwrap();
        let set = map_order.entry(key.parse().unwrap()).or_insert(HashSet::new());
        set.insert(value.parse().unwrap());
    }
    updates = updates_string.lines().map(|line| line.split(',').map(|s| s.parse().unwrap()).collect()).collect();

    (map_order, updates)
}
fn sum_middles(map_order: &HashMap<u64, HashSet<u64>>, updates: &Vec<Vec<u64>>) -> (u64, u64) {
    let mut valid_middles = 0;
    let mut invalid_middles = 0;
    for update in updates {
        let mut sorted_update = update.clone();
        sorted_update.sort_by(|a, b| {
            if let Some(set) = map_order.get(a) {
                if set.contains(b) { 
                    return Ordering::Less;
                }
            }
            Ordering::Greater
        });
        if *update == sorted_update {
            valid_middles += update[update.len() / 2];
        } else  {
            invalid_middles += sorted_update[sorted_update.len() / 2];
        }
    }
    (valid_middles, invalid_middles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_middles() {
        let (map_order, updates) = read_input("test/test05.txt");
        assert_eq!(sum_middles(&map_order, &updates), (143, 123));
    }
}
