use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use regex::Regex;
///////////////////////////////////////////////////////////////////////////////
pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol2: u64 = 0;
    let mut robots = parse_input("input/day14.txt");
    simulate_robots(&mut robots, 100);

    (Solution::from(get_count_of_robots_in_quadrant(&robots)), Solution::from(sol2))
}
fn get_count_of_robots_in_quadrant(robots: &Vec<Robot>) -> u64 {
    let half_x = MAX_X / 2;
    let half_y = MAX_Y / 2;
    let tl = robots.iter().filter(|robot| robot.current_position.0 < half_x && robot.current_position.1 < half_y).count() as u64;
    let tr = robots.iter().filter(|robot| robot.current_position.0 > half_x && robot.current_position.1 < half_y).count() as u64;
    let bl = robots.iter().filter(|robot| robot.current_position.0 < half_x && robot.current_position.1 > half_y).count() as u64;
    let br = robots.iter().filter(|robot| robot.current_position.0 > half_x && robot.current_position.1 > half_y).count() as u64;
    tl * tr * bl * br
}

const MAX_X:i64 = 101;
const MAX_Y:i64 = 103;
#[derive(Debug)]
struct Robot {
    current_position: (i64, i64),
    velocity: (i64, i64),
}
impl Robot {
    fn take_step(&mut self) {
        self.current_position.0 = (self.velocity.0 + self.current_position.0) % MAX_X;
        self.current_position.1 = (self.velocity.1 + self.current_position.1) % MAX_Y;
        if self.current_position.0 < 0 {
            self.current_position.0 += MAX_X;
        }
        if self.current_position.1 < 0 {
            self.current_position.1 += MAX_Y;
        }
    }
}

fn simulate_robots(robots: &mut Vec<Robot>, steps:i64) {
    for _ in 0..steps {
        for robot in robots.iter_mut() {
            robot.take_step();
        }
    }
}

fn parse_input(file_name: &str) -> Vec<Robot> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();
    let input = read_to_string(file_name).unwrap();
    let robots = input.lines().map(|line| {
        let (_, [current_x, current_y, velocity_x, velocity_y]) = regex.captures(line).unwrap().extract();
        Robot {
            current_position: (current_x.parse().unwrap(), current_y.parse().unwrap()),
            velocity: (velocity_x.parse().unwrap(), velocity_y.parse().unwrap()),
        }
    }).collect();
    robots
}