use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let equations = read_input("input/day07.txt");
    let sol1: u64 = find_valid_equations_part1(equations.clone());
    let sol2: u64 = find_valid_equations_part2(equations);

    (Solution::from(sol1), Solution::from(sol2))
}
fn find_valid_equations_part1(equations: Vec<(u64, Vec<u64>)>) -> u64 {
    let mut valid_equations = 0;
    for (answer, mut numbers) in equations {
        let first_number = numbers.remove(0);
        if let Some(result) = operate(answer, first_number, numbers.clone()) {
            valid_equations += result;
        }
    }
    valid_equations
}

fn operate(answer: u64, left: u64, mut numbers: Vec<u64>) -> Option<u64> {
    if numbers.len() == 0 {
        Some(left)
    } else {
        let next_number = numbers.remove(0);
        let final_add = operate(answer, left + next_number, numbers.clone());
        let final_mult = operate(answer, left * next_number, numbers.clone());

        if final_add.is_some() && final_add.unwrap() == answer {
            final_add
        } else if final_mult.is_some() && final_mult.unwrap() == answer {
            final_mult
        } else {
            None
        }
    }
}

fn find_valid_equations_part2(equations: Vec<(u64, Vec<u64>)>) -> u64 {
    let mut valid_equations = 0;
    for (answer, mut numbers) in equations {
        let first_number = numbers.remove(0);
        if let Some(result) = operate_part2(answer, first_number, numbers.clone()) {
            valid_equations += result;
        }
    }
    valid_equations
}

fn operate_part2(answer: u64, left: u64, mut numbers: Vec<u64>) -> Option<u64> {
    if numbers.len() == 0 {
        Some(left)
    } else {
        let next_number = numbers.remove(0);
        let final_add = operate_part2(answer, left + next_number, numbers.clone());
        let final_mult = operate_part2(answer, left * next_number, numbers.clone());
        let final_concat = operate_part2(answer, format!("{}{}", left, next_number).parse::<u64>().unwrap(), numbers.clone());

        if final_add.is_some() && final_add.unwrap() == answer {
            final_add
        } else if final_mult.is_some() && final_mult.unwrap() == answer {
            final_mult
        } else if final_concat.is_some() && final_concat.unwrap() == answer {
            final_concat
        } else {
            None
        }
    }
}

fn read_input(filename: &str) -> Vec<(u64, Vec<u64>)> {
    let contents = read_to_string(filename).unwrap();
    contents.lines().map(parse_line).collect()
}
fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let answer = parts[0].parse::<u64>().unwrap();
    let numbers: Vec<u64> = parts[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    (answer, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_valid_equations_part1() {
        let equations = read_input("test/test07.txt");
        let valid_equations = find_valid_equations_part1(equations);
        assert_eq!(valid_equations, 3749);
    }

    #[test]
    fn test_find_valid_equations_part2() {
        let equations = read_input("test/test07.txt");
        let valid_equations = find_valid_equations_part2(equations);
        assert_eq!(valid_equations, 11387);
    }

    #[test]
    fn test_operate() {
        let result = operate(190, 10, vec![19]);
        assert_eq!(result, Some(190));

        let result = operate(3267, 81, vec![40,27]);
        assert_eq!(result, Some(3267))
    }

    #[test]
    fn test_operate_part2() {
        let result = operate_part2(156, 15, vec![6]);
        assert_eq!(result, Some(156));

        let result = operate_part2(7290, 6, vec![8, 6, 15]);
        assert_eq!(result, Some(7290));
    }
}
