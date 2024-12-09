use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/input03.txt").unwrap();
    let sol1 = find_mul_ops(&input);
    let sol2 = find_mul_ops_do_or_do_not(&input);
    (Solution::from(sol1), Solution::from(sol2))
}
fn find_mul_ops(input: &str) -> i64 {
    let regex = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [left, right]) in regex.captures_iter(input).map(|c| c.extract()) {
        sum += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
    }
    sum
}
fn find_mul_ops_do_or_do_not(input: &str) -> i64 {
    let regex = Regex::new(r"(do\(\)|don't\(\))|(mul\(([\d]{1,3}),([\d]{1,3})\))").unwrap();
    let mut sum = 0;
    let mut should_multiply: bool = true;
    for capture in regex.captures_iter(input) {
        let sub_str = capture.get(0).unwrap().as_str();
        if sub_str == "do()" {
            should_multiply = true;
            continue;
        } else if sub_str == "don't()" {
            should_multiply = false;
            continue;
        }
        if !should_multiply {
            continue;
        }
        let left = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let right = capture.get(4).unwrap().as_str().parse::<i64>().unwrap();
        sum += left * right;
    }
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mul_ops() {
        assert_eq!(
            find_mul_ops("mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }
    #[test]
    fn test_find_mul_ops_do_or_do_not() {
        assert_eq!(
            find_mul_ops_do_or_do_not(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
