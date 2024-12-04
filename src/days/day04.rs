use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let array = read_input("input/day04.txt");
    let sol1 = count_xmas_words(&array);
    let sol2 = count_x_words(&array);

    (Solution::from(sol1), Solution::from(sol2))
}
pub fn read_input(file_name: &str) -> Vec<Vec<char>> {
    let input = read_to_string(file_name).unwrap();
    input.lines().map(|line| line.chars().collect()).collect()
}
const DIRECTIONS: [(i64, i64); 8] = [
    (0, 1), (1, 0), (0, -1), (-1, 0),
    (1, 1), (-1, -1), (1, -1), (-1, 1),
];
pub fn count_xmas_words(input: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'X' {
                // time for that xmas spirit
                for direction in DIRECTIONS {
                    if search_for_next_char(&input, i,j, direction, 1) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
pub fn search_for_next_char(input: &Vec<Vec<char>>, i: usize, j: usize, direction: (i64, i64), distance: i64) -> bool {
    let new_i = i as i64 + direction.0;
    let new_j = j as i64 + direction.1;
    if new_i < 0 || new_i >= input.len() as i64 || new_j < 0 || new_j >= input[new_i as usize].len() as i64 {
        return false;
    }

    match (distance,input[new_i as usize][new_j as usize]) {
        (1, 'M') => search_for_next_char(input, new_i as usize, new_j as usize, direction, distance+1),
        (2, 'A') => search_for_next_char(input, new_i as usize, new_j as usize, direction, distance+1),
        (3, 'S') => true,
        (_, _) => false,
    }
}

pub fn count_x_words(input: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'A' {
                // only interior A's count
                if !(i == 0 || i == input.len() - 1 || j == 0 || j == input[i].len() - 1) {
                    let diag1 = format!("{}{}{}", input[i-1][j-1], input[i][j], input[i+1][j+1]);
                    let diag2 = format!("{}{}{}", input[i+1][j-1], input[i][j], input[i-1][j+1]);
                    if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_xmas_words() {
        let input = read_input("test/test04.txt");
        assert_eq!(count_xmas_words(&input), 18);
    }
    #[test]
    fn test_count_x_words() {
        let input = read_input("test/test04-2.txt");
        assert_eq!(count_x_words(&input), 9);
    }
}   