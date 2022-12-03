use crate::utils::constants::SPACE;
use std::str::FromStr;

#[derive(Debug,PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = std::string::ParseError;

    fn from_str(choise_str: &str) -> Result<Self, Self::Err> {
        let choise = match choise_str {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unreachable!("Invalid shape"),
        };

        Ok(choise)
    }
}

fn parse_line(line: &str) -> (RPS, RPS) {
    let (l, r) = line.split_once(SPACE).unwrap();

    (RPS::from_str(l).unwrap(), RPS::from_str(r).unwrap())
}

fn parse_input(input: &String) -> Vec<(RPS, RPS)> {
    input.trim().lines().map(parse_line).collect()
}

fn get_socre(game: &(RPS, RPS)) -> u64 {
    match game {
        (RPS::Rock,     RPS::Rock)      => 4, // 1 + 3,
        (RPS::Rock,     RPS::Paper)     => 8, // 2 + 6,
        (RPS::Rock,     RPS::Scissors)  => 3, // 3 + 0,

        (RPS::Paper,    RPS::Rock)      => 1, // 1 + 0,
        (RPS::Paper,    RPS::Paper)     => 5, // 2 + 3,
        (RPS::Paper,    RPS::Scissors)  => 9, // 3 + 6,

        (RPS::Scissors, RPS::Rock)      => 7, // 1 + 6,
        (RPS::Scissors, RPS::Paper)     => 2, // 2 + 0,
        (RPS::Scissors, RPS::Scissors)  => 6, // 3 + 3,
    }
}

pub fn part_1(input: &String) -> u64 {
    parse_input(input).iter().map(get_socre).sum()
}

pub fn part_2(input: &String) -> u64 {
    11
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn parses_first_elf_choise() {
        assert_eq!(RPS::from_str("A"), Ok(RPS::Rock));
        assert_eq!(RPS::from_str("B"), Ok(RPS::Paper));
        assert_eq!(RPS::from_str("C"), Ok(RPS::Scissors));
    }

    #[test]
    fn parses_second_elf_choise() {
        assert_eq!(RPS::from_str("X"), Ok(RPS::Rock));
        assert_eq!(RPS::from_str("Y"), Ok(RPS::Paper));
        assert_eq!(RPS::from_str("Z"), Ok(RPS::Scissors));
    }

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&String::from(TEST_INPUT)), 15);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&String::from(TEST_INPUT)), 10);
    }
}
