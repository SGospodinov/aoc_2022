fn parse_elf_calories(raw_calories: &str) -> Vec<u64> {
    raw_calories.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

fn get_elv_calories(input: &String) -> Vec<u64> {
    input
        .split("\n\n")
        .map(parse_elf_calories)
        .map(|elf_calories| elf_calories.iter().sum())
        .collect()
}

pub fn part_1(input: &String) -> u64 {
    get_elv_calories(input).into_iter().max().unwrap()
}

pub fn part_2(input: &String) -> u64 {
    let mut elf_calories = get_elv_calories(input);
    elf_calories.sort();

    elf_calories.iter().rev().take(3).sum::<u64>()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&String::from(TEST_INPUT)), 24000);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&String::from(TEST_INPUT)), 45000);
    }
}
