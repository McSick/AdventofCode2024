use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;
    let map = read_input("input/day06.txt");
    let sol1 = solve1(map.clone());
    let sol2 = solve2(map);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(mut map: Map) -> usize {
    loop {
        if map.try_move_guard_out() != State::Moving {
            break;
        }
    }
    map.grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| tile.visited)
        .count()
}

fn solve2(mut map: Map) -> usize {
    let mut count = 0;
    let mut positions_to_try = Vec::new();
    let mut map_copy = map.clone();
    let starting_position = map.guard.location.clone();
    loop {
        if map_copy.try_move_guard_out() != State::Moving {
            break;
        }
    }
    for i in 0..map_copy.grid.len() {
        for j in 0..map_copy.grid[0].len() {
            if starting_position != (i, j) && map_copy.grid[i][j].visited {
                positions_to_try.push((i, j));
            }
        }
    }

    for position in positions_to_try {
        let mut map_copy = map.clone();
        map_copy.grid[position.0][position.1].visited = false;
        map_copy.grid[position.0][position.1].tile_type = TileType::Obstacle;
        let mut num_iterations = 0;
        loop {
            let state = map_copy.try_move_guard_out();
            if state != State::Moving {
                if state == State::Looping {
                    count += 1;
                }
                break;
            }
        }
    }
    count
}
type Grid = Vec<Vec<Tile>>;
#[derive(Clone)]
struct Map {
    grid: Grid,
    guard: Guard,
    visited_positions: HashSet<(usize, usize, Direction)>,
}
#[derive(PartialEq, Debug)]
enum State {
    Moving,
    Out,
    Looping,
}
impl Map {
    fn get_tile(&self, location: (usize, usize)) -> (&Tile, bool) {
        (&self.grid[location.0][location.1], self.is_edge(location))
    }
    fn is_edge(&self, location: (usize, usize)) -> bool {
        location.0 == 0
            || location.1 == 0
            || location.0 == self.grid.len() - 1
            || location.1 == self.grid[0].len() - 1
    }
    fn try_move_guard_out(&mut self) -> State {
        if self.will_move_out_of_bounds() {
            return State::Out;
        }
        if self.will_hit_obstacle() {
            self.turn_guard();
            if self.visited_positions.contains(&(
                self.guard.location.0,
                self.guard.location.1,
                self.guard.direction.clone(),
            )) {
                return State::Looping;
            } else {
                self.visited_positions.insert((
                    self.guard.location.0,
                    self.guard.location.1,
                    self.guard.direction.clone(),
                ));
            }
            return State::Moving;
        }
        self.move_guard();
        State::Moving
    }
    fn turn_guard(&mut self) {
        match self.guard.direction {
            Direction::Up => self.guard.direction = Direction::Right,
            Direction::Right => self.guard.direction = Direction::Down,
            Direction::Down => self.guard.direction = Direction::Left,
            Direction::Left => self.guard.direction = Direction::Up,
        }
    }
    fn move_guard(&mut self) {
        match self.guard.direction {
            Direction::Up => self.guard.location.0 -= 1,
            Direction::Right => self.guard.location.1 += 1,
            Direction::Down => self.guard.location.0 += 1,
            Direction::Left => self.guard.location.1 -= 1,
        }
        self.grid[self.guard.location.0][self.guard.location.1].visited = true;
    }
    fn will_hit_obstacle(&self) -> bool {
        match self.guard.direction {
            Direction::Up => {
                self.grid[self.guard.location.0 - 1][self.guard.location.1].tile_type
                    == TileType::Obstacle
            }
            Direction::Right => {
                self.grid[self.guard.location.0][self.guard.location.1 + 1].tile_type
                    == TileType::Obstacle
            }
            Direction::Down => {
                self.grid[self.guard.location.0 + 1][self.guard.location.1].tile_type
                    == TileType::Obstacle
            }
            Direction::Left => {
                self.grid[self.guard.location.0][self.guard.location.1 - 1].tile_type
                    == TileType::Obstacle
            }
        }
    }
    fn will_move_out_of_bounds(&self) -> bool {
        match self.guard.direction {
            Direction::Up => self.guard.location.0 == 0,
            Direction::Right => self.guard.location.1 == self.grid[0].len() - 1,
            Direction::Down => self.guard.location.0 == self.grid.len() - 1,
            Direction::Left => self.guard.location.1 == 0,
        }
    }
}
#[derive(Clone)]
struct Tile {
    tile_type: TileType,
    visited: bool,
}
impl Tile {
    fn new(c: char) -> Tile {
        let visited = match c {
            '^' | '>' | '<' | 'v' => true,
            _ => false,
        };
        Tile {
            tile_type: TileType::from_char(c),
            visited: visited,
        }
    }
}
#[derive(PartialEq, Clone)]
enum TileType {
    Empty,
    Obstacle,
}
#[derive(PartialEq, Clone, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone)]
struct Guard {
    location: (usize, usize),
    direction: Direction,
}
impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            '#' => TileType::Obstacle,
            _ => TileType::Empty,
        }
    }
}

fn read_input(file_name: &str) -> Map {
    let text = read_to_string(&file_name).unwrap();
    let mut some_guard: Option<Guard> = None;
    let mut grid: Grid = Vec::new();
    for (i, line) in text.lines().enumerate() {
        grid.push(Vec::new());
        for (j, c) in line.chars().enumerate() {
            grid[i].push(Tile::new(c));
            match c {
                '^' => {
                    some_guard = Some(Guard {
                        location: (i, j),
                        direction: Direction::Up,
                    })
                }
                'v' => {
                    some_guard = Some(Guard {
                        location: (i, j),
                        direction: Direction::Down,
                    })
                }
                '<' => {
                    some_guard = Some(Guard {
                        location: (i, j),
                        direction: Direction::Left,
                    })
                }
                '>' => {
                    some_guard = Some(Guard {
                        location: (i, j),
                        direction: Direction::Right,
                    })
                }
                _ => (),
            }
        }
    }
    let guard = some_guard.unwrap();
    let mut visited_positions = HashSet::new();
    visited_positions.insert((
        guard.clone().location.0,
        guard.clone().location.1,
        guard.clone().direction,
    ));
    Map {
        grid: grid,
        guard: guard,
        visited_positions: visited_positions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic() {
        let map = read_input("test/test06.txt");

        let count = solve1(map);

        assert_eq!(count, 41);
    }

    #[test]
    fn test_logic2() {
        let map = read_input("test/test06.txt");
        let count = solve2(map);
        assert_eq!(count, 6);
    }
}
