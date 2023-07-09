use aoc_4;
use std::fs::{self};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input-1.txt").unwrap();
    let total_overlapped = aoc_4::process_part_1(&input);
    println!("part 1: {}", total_overlapped);
}

fn part_2() {
    let input = fs::read_to_string("input-2.txt").unwrap();
    let total_part_overlapped = aoc_4::process_part_2(&input);
    println!("part 2: {}", total_part_overlapped);
}
