use crate::{Solution, SolutionPair};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////
pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol2: u64 = 0;
    let mut robots = parse_input("input/day14.txt");
    let mut robots_pt2 = robots.clone();
    let mut robots_printable = robots.clone();
    simulate_robots(&mut robots, 100);
    let mut tree_step = find_tree_step(&mut robots_pt2);
    simulate_robots(&mut robots_printable, tree_step);
    print_robots(&robots_printable);

    (
        Solution::from(get_count_of_robots_in_quadrant(&robots)),
        Solution::from(tree_step),
    )
}
fn get_count_of_robots_in_quadrant(robots: &Vec<Robot>) -> u64 {
    let half_x = MAX_X / 2;
    let half_y = MAX_Y / 2;
    let tl = robots
        .iter()
        .filter(|robot| robot.current_position.0 < half_x && robot.current_position.1 < half_y)
        .count() as u64;
    let tr = robots
        .iter()
        .filter(|robot| robot.current_position.0 > half_x && robot.current_position.1 < half_y)
        .count() as u64;
    let bl = robots
        .iter()
        .filter(|robot| robot.current_position.0 < half_x && robot.current_position.1 > half_y)
        .count() as u64;
    let br = robots
        .iter()
        .filter(|robot| robot.current_position.0 > half_x && robot.current_position.1 > half_y)
        .count() as u64;
    tl * tr * bl * br
}

const MAX_X: i64 = 101;
const MAX_Y: i64 = 103;
#[derive(Debug, Clone)]
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

fn simulate_robots(robots: &mut Vec<Robot>, steps: i64) {
    for _ in 0..steps {
        for robot in robots.iter_mut() {
            robot.take_step();
        }
    }
}
// 5 is probably fine. Robots should be close to each other in a picture
const CLUSTER_SIZE: i64 = 5;
fn biggest_cluster_size(robots: &mut Vec<Robot>) -> u64 {
    let mut max_count = 0;

    let mut clusters = HashMap::new();
    for robot in robots.iter() {
        let x = robot.current_position.0 / CLUSTER_SIZE;
        let y = robot.current_position.1 / CLUSTER_SIZE;
        let cluster = clusters.entry((x, y)).or_insert(0);
        *cluster += 1;
    }

    let max_cluster = clusters.iter().max_by_key(|&(_, count)| count).unwrap();
    *max_cluster.1
}

fn find_tree_step(robots: &mut Vec<Robot>) -> i64 {
    let mut max_step = 0;
    let mut max_count = 0;

    for step in 1..(MAX_X * MAX_Y) {
        simulate_robots(robots, 1);
        let counts = biggest_cluster_size(robots);

        if counts > max_count {
            max_count = counts;
            max_step = step;
        }
    }
    max_step
}

fn print_robots(robots: &Vec<Robot>) {
    println!("");
    for y in 0..MAX_Y {
        for x in 0..MAX_X {
            if robots
                .iter()
                .any(|robot| robot.current_position.0 == x && robot.current_position.1 == y)
            {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
fn parse_input(file_name: &str) -> Vec<Robot> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();
    let input = read_to_string(file_name).unwrap();
    let robots = input
        .lines()
        .map(|line| {
            let (_, [current_x, current_y, velocity_x, velocity_y]) =
                regex.captures(line).unwrap().extract();
            Robot {
                current_position: (current_x.parse().unwrap(), current_y.parse().unwrap()),
                velocity: (velocity_x.parse().unwrap(), velocity_y.parse().unwrap()),
            }
        })
        .collect();
    robots
}
