use std::{str::FromStr, string::ParseError};


trait Score {
    fn get_score(&self) -> i32;
}

enum Play {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Play {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => panic!("failed to match {s}")
        }
    }
}

impl Score for Play {
    fn get_score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input-2.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let result = input.lines().map(calculate_match_1);
    println!("{}", result.sum::<i32>());
}

fn part2(input: &String) {
    let result = input.lines().map(calculate_match_2);
    println!("{}", result.sum::<i32>());
}

fn calculate_match_1(current_match: &str) -> i32 {
    let plays = current_match.split(" ").collect::<Vec<&str>>();
    let oponent = Play::from_str(plays[0]).unwrap();
    let player = Play::from_str(plays[1]).unwrap();
    let op_score = oponent.get_score();
    let p_score = player.get_score();

    if (op_score == 3 && p_score == 1) || (op_score + 1 == p_score) {
        return 6 + p_score;
    } else if op_score == p_score {
        return 3 + p_score;
    } else {
        return p_score;
    }
}

fn calculate_match_2(current_match: &str) -> i32 {
    let plays = current_match.split(" ").collect::<Vec<&str>>();
    let oponent = Play::from_str(plays[0]).unwrap();
    let op_score = oponent.get_score();

    if plays[1] == "X" {
        return  (op_score - 2).rem_euclid(3) + 1;
    } else if plays[1] == "Y" {
        return 3 + op_score;
    } else {
        return  6 + (op_score % 3) + 1;
    }
}

