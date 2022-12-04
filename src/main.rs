#![feature(iter_next_chunk)]

mod utils;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

use crate::utils::load_day_input;

fn main() {
    println!("🎄 Advent of Code 2022 🎁");

    println!("Day 1 - Part 1: {}", day_1::part_1(&load_day_input(1)));
    println!("Day 1 - Part 2: {}", day_1::part_2(&load_day_input(1)));
    println!("Day 2 - Part 1: {}", day_2::part_1(&load_day_input(2)));
    println!("Day 2 - Part 2: {}", day_2::part_2(&load_day_input(2)));
    println!("Day 3 - Part 1: {}", day_3::part_1(&load_day_input(3)));
    println!("Day 3 - Part 2: {}", day_3::part_2(&load_day_input(3)));
    println!("Day 4 - Part 1: {}", day_4::part_1(&load_day_input(4)));
    println!("Day 4 - Part 2: {}", day_4::part_2(&load_day_input(4)));
}
