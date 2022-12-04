use crate::utils::constants::{COMMA, HYPHEN};
use std::collections::HashSet;

fn create_section_set(section_range_str: &str) -> HashSet<u32> {
    let (start, end) = section_range_str.split_once(HYPHEN).unwrap();

    let start = start.parse::<u32>().unwrap();
    let end = end.parse::<u32>().unwrap();

    (start..=end).into_iter().collect()
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (left, right) = line.split_once(COMMA).unwrap();

    (create_section_set(left), create_section_set(right))
}

fn fully_contains((left, right): &(HashSet<u32>, HashSet<u32>)) -> bool {
    left.is_subset(&right) || right.is_subset(&left)
}


pub fn part_1(input: &String) -> u64 {
    input.trim().lines().map(parse_line).filter(fully_contains).count() as u64
}

fn overlaps((left, right): &(HashSet<u32>, HashSet<u32>)) -> bool {
    !left.is_disjoint(&right)
}

pub fn part_2(input: &String) -> u64 {
    input.trim().lines().map(parse_line).filter(overlaps).count() as u64
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&String::from(TEST_INPUT)), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&String::from(TEST_INPUT)), 4);
    }
}
