mod utils;
mod day_1;

use crate::utils::load_day_input;

fn main() {
    println!("🎄 Advent of Code 2022 🎁");

    println!("Day 1 - Part 1: {}", day_1::part_1(&load_day_input(1)));
    println!("Day 1 - Part 2: {}", day_1::part_2(&load_day_input(1)));
}
