use std::collections::HashSet;

fn get_priority(item: char) -> u64 {
    match item {
        'a'..='z' => item as u64 - 'a' as u64 + 1,
        'A'..='Z' => item as u64 - 'A' as u64 + 27,
        _         => unreachable!(),
    }
}

fn find_worng_item(rucksack: &str) -> char {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);

    let first = first.chars().into_iter().collect::<HashSet<char>>();
    let second = second.chars().into_iter().collect::<HashSet<char>>();

    *first.intersection(&second).nth(0).unwrap()
}

fn get_wrong_item_priority(rucksack: &str) -> u64 {
    get_priority(find_worng_item(rucksack))
}

pub fn part_1(input: &String) -> u64 {
    input.trim().lines().map(get_wrong_item_priority).sum()
}

pub fn part_2(input: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&String::from(TEST_INPUT)), 157);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&String::from(TEST_INPUT)), 1);
    }
}
