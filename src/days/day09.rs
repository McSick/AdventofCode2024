use crate::{Solution, SolutionPair};
use std::fmt::Debug;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...

    let storage = read_input("input/day09.txt");
    let storage_part2 = storage.clone();

    let storage = shrink_storage(storage);
    let sol1 = find_check_sum(storage);

    let storage_part2 = shrink_storage_by_block(storage_part2);
    let sol2 = find_check_sum(storage_part2);

    (Solution::from(sol1), Solution::from(sol2))
}
fn shrink_storage(mut storage: Vec<StorageSpace>) -> Vec<StorageSpace> {
    let mut free_index = 0;
    free_index = find_next_free_space(free_index, &storage);
    let mut block_index = storage.len() - 1;
    block_index = find_next_block_space(block_index, &storage);

    while free_index < block_index {
        // swap free and block
        storage.swap(free_index, block_index);
        free_index = find_next_free_space(free_index, &storage);
        block_index = find_next_block_space(block_index, &storage);
    }
    storage
}
fn shrink_storage_by_block(mut storage: Vec<StorageSpace>) -> Vec<StorageSpace> {
    let block_index = find_next_block_space(storage.len() - 1, &storage);
    let mut block_size = 0;
    let mut block_id = 0;
    if let StorageSpace::ID(id) = storage[block_index] {
        block_id = id;
    }
    loop {
        let mut free_index = find_next_free_space(0, &storage);
        let mut free_size = 0;
        let pos_block_index = storage
            .iter()
            .position(|x| *x == StorageSpace::ID(block_id))
            .unwrap();
        block_size = storage
            .iter()
            .filter(|&n| *n == StorageSpace::ID(block_id))
            .count();

        let mut is_free_found = false;
        loop {
            (free_index, free_size) = find_next_free_size(free_index, &storage);
            if free_index <= pos_block_index
                && free_size >= block_size
                && free_index < storage.len()
            {
                is_free_found = true;
                break;
            }
            free_index = find_next_free_space(free_index + 1, &storage);
            if free_index >= storage.len() {
                break;
            }
        }
        if is_free_found && free_size >= block_size {
            let block = storage[pos_block_index];
            for i in 0..block_size {
                storage[free_index + i - free_size] = block;
                storage[pos_block_index + i] = StorageSpace::Free;
            }
        }
        if block_id > 0 {
            block_id -= 1;
        } else {
            break;
        }
    }
    storage
}
fn find_next_free_size(free_index: usize, storage: &Vec<StorageSpace>) -> (usize, usize) {
    let mut free_index = free_index;
    let mut size = 0;
    while free_index < storage.len() && storage[free_index] == StorageSpace::Free {
        free_index += 1;
        size += 1;
    }
    (free_index, size)
}

fn find_next_block_size(
    block_index: usize,
    block: StorageSpace,
    storage: &Vec<StorageSpace>,
) -> usize {
    let mut block_index = block_index;
    let mut size = 1;
    while block_index < (storage.len() - 1) && storage[block_index] == block {
        block_index += 1;
        size += 1;
    }
    size
}
fn find_check_sum(storage: Vec<StorageSpace>) -> u64 {
    let mut sum = 0;
    for (i, space) in storage.iter().enumerate() {
        if let StorageSpace::ID(num) = space {
            sum += num * (i as u64);
        }
    }
    sum
}

fn find_next_free_space(free_index: usize, storage: &Vec<StorageSpace>) -> usize {
    let mut free_index = free_index;
    while free_index < storage.len() && storage[free_index] != StorageSpace::Free {
        free_index += 1;
    }
    free_index
}
fn find_next_block_space(block_index: usize, storage: &Vec<StorageSpace>) -> usize {
    let mut block_index = block_index;
    while storage[block_index] == StorageSpace::Free {
        block_index -= 1;
    }
    block_index
}

//2333133121414131402
// alternate free, uuid, and build out giant vector of all the numbers
#[derive(PartialEq, Clone, Copy)]
enum StorageSpace {
    Free,
    ID(u64),
}
impl Debug for StorageSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            StorageSpace::ID(x) => write!(f, "{}", x),
            StorageSpace::Free => write!(f, "."),
        }
    }
}

fn read_input(file_name: &str) -> Vec<StorageSpace> {
    let file_string = read_to_string(file_name).unwrap();
    let mut storage = Vec::new();
    let mut is_block = true;
    let mut id = 0;
    for line in file_string.lines() {
        for c in line.chars() {
            let num = c.to_digit(10).unwrap();
            if is_block {
                for _ in 0..num {
                    storage.push(StorageSpace::ID(id));
                }
                id += 1;
            } else {
                for _ in 0..num {
                    storage.push(StorageSpace::Free);
                }
            }
            is_block = !is_block;
        }
    }
    storage
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_check_sum() {
        let storage = read_input("test/test09.txt");
        println!("{:?}", storage);
        let storage = shrink_storage(storage);
        println!("{:?}", storage);
        let check_sum = find_check_sum(storage);
        assert_eq!(check_sum, 1928);
    }
    #[test]
    fn test_shrink_storage_by_block() {
        let storage = read_input("test/test09.txt");
        let storage = shrink_storage_by_block(storage);
        println!("{:?}", storage);
        let check_sum = find_check_sum(storage);

        let storage = read_input("test/test09-2.txt");
        let storage = shrink_storage_by_block(storage);
        println!("{:?}", storage);
        let check_sum = find_check_sum(storage);

        assert_eq!(check_sum, 16);
    }
}
