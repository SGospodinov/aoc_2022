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

fn get_all_items(rucksack: &str) -> HashSet<char> {
    rucksack.chars().collect::<HashSet<char>>()
}

pub fn part_2(input: &String) -> u64 {
    let mut result = 0;
    let mut lines = input.trim().lines();

    while let Ok(group) = lines.next_chunk::<3>() {
        let one   = get_all_items(group[0]);
        let two   = get_all_items(group[1]);
        let three = get_all_items(group[2]);

        let badge: HashSet<_> = one.intersection(&two).cloned().collect();
        let badge = badge.intersection(&three).nth(0).unwrap();

        result += get_priority(*badge);
    }

    result
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
        assert_eq!(super::part_2(&String::from(TEST_INPUT)), 70);
    }
}
