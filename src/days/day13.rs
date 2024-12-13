use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use regex::Regex;
use std::cmp::min;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let machine_list = read_input("input/day13.txt");
    let (cost, cost2) = count_all_the_machines(&machine_list);

    (Solution::from(cost), Solution::from(cost2))
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Button {
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Machine {
    buttonA: Button,
    buttonB: Button,
    x: i64,
    y: i64,
}
const A_COST: i64 = 3;
const B_COST: i64 = 1;
const MAX_PRESSES: i64 = 100;
fn count_all_the_machines(machine_list: &Vec<Machine>) -> (i64, i64) {
    let mut total_cost = 0;
    let mut total_cost2 = 0;
    for machine in machine_list {
        if let Some(cost) = try_to_win_prize(machine) {
            total_cost += cost;
        }
        let mut machine2 = machine.clone();
        machine2.x += 10000000000000;
        machine2.y += 10000000000000;
        if let Some(cost) = try_to_win_prize(&machine2) {
            total_cost2 += cost;
        }
    }
    (total_cost, total_cost2)
}
fn try_to_win_prize(machine: &Machine) -> Option<i64> {
    let a_presses = (machine.x * machine.buttonB.y - machine.y *machine.buttonB.x) / (machine.buttonA.x * machine.buttonB.y - machine.buttonA.y * machine.buttonB.x);
    let b_presses = (machine.y * machine.buttonA.x - machine.x * machine.buttonA.y) /(machine.buttonA.x * machine.buttonB.y - machine.buttonA.y * machine.buttonB.x);
    if (machine.buttonA.x * a_presses + machine.buttonB.x * b_presses) == machine.x && (machine.buttonA.y * a_presses + machine.buttonB.y * b_presses) == machine.y {
        return Some(a_presses * A_COST + b_presses * B_COST);
    }
    return None;
}
fn read_input(file_name: &str) -> Vec<Machine> {
    let input = read_to_string(file_name).unwrap();
    let machine_list = input.split("\n\n").map(parse_machine).collect::<Vec<Machine>>();
    machine_list
}
fn parse_machine(block: &str) -> Machine {
    let lines = block.split("\n").collect::<Vec<&str>>();
    let button_regex = Regex::new(r"Button (A|B): X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let (_, [_button, a_x,a_y]) = button_regex.captures(lines[0]).unwrap().extract();
    let (_, [_button, b_x,b_y]) = button_regex.captures(lines[1]).unwrap().extract();
    let prize_regex= Regex::new(r"Prize: X=([\d]+), Y=([\d]+)").unwrap();
    let (_, [x,y]) = prize_regex.captures(lines[2]).unwrap().extract();
    Machine {
        buttonA: Button { x: a_x.parse().unwrap(), y: a_y.parse().unwrap() },
        buttonB: Button { x: b_x.parse().unwrap(), y: b_y.parse().unwrap() },
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_machine() {
        let machine_list = read_input("test/test13.txt");
        println!("{:?}", machine_list);
        assert_eq!(machine_list.len(), 4);
    }
}