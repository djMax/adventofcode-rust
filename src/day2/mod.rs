use std::include_str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Choice {
  Rock,
  Paper,
  Scissors,
}

enum MatchResult {
  Win,
  Lose,
  Draw,
}

struct Round {
  me: Choice,
  them: Choice,
}

fn round_result(round: &Round) -> MatchResult {
  match round.me {
    Choice::Rock => match round.them {
      Choice::Rock => MatchResult::Draw,
      Choice::Paper => MatchResult::Lose,
      Choice::Scissors => MatchResult::Win,
    },
    Choice::Paper => match round.them {
      Choice::Rock => MatchResult::Win,
      Choice::Paper => MatchResult::Draw,
      Choice::Scissors => MatchResult::Lose,
    },
    Choice::Scissors => match round.them {
      Choice::Rock => MatchResult::Lose,
      Choice::Paper => MatchResult::Win,
      Choice::Scissors => MatchResult::Draw,
    },
  }
}

fn shape_score(c: &Choice) -> i32 {
  return match c {
    Choice::Rock => 1,
    Choice::Paper => 2,
    Choice::Scissors => 3,
  }
}

fn round_score(result: &MatchResult) -> i32 {
  return match result {
    MatchResult::Win => 6,
    MatchResult::Draw => 3,
    MatchResult::Lose => 0,
  }
}

fn matching_choice(round: &Round) -> Choice {
  let goal = match round.me {
    Choice::Rock => MatchResult::Lose,
    Choice::Paper => MatchResult::Draw,
    Choice::Scissors => MatchResult::Win,
  };
  return match round.them {
    Choice::Rock => match goal {
      MatchResult::Win => Choice::Paper,
      MatchResult::Draw => Choice::Rock,
      MatchResult::Lose => Choice::Scissors,
    },
    Choice::Paper => match goal {
      MatchResult::Win => Choice::Scissors,
      MatchResult::Draw => Choice::Paper,
      MatchResult::Lose => Choice::Rock,
    },
    Choice::Scissors => match goal {
      MatchResult::Win => Choice::Rock,
      MatchResult::Draw => Choice::Scissors,
      MatchResult::Lose => Choice::Paper,
    },
  }
}

fn parse(input: &str) -> Vec<Round> {
  return input
    .trim()
    .split("\n")
    .map(|group| {
      let them = match group.chars().nth(0).unwrap_or(' ') {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => panic!("Invalid choice for them"),
      };
      let me = match group.chars().nth(2).unwrap_or(' ') {
        'X' => Choice::Rock,
        'Y' => Choice::Paper,
        'Z' => Choice::Scissors,
        _ => panic!("Invalid choice for me"),
      };
      return Round { me, them };
    })
    .collect();
}

fn part1(input: &Vec<Round>) -> i32 {
  return input.iter()
    .map(|round| shape_score(&round.me) + round_score(&round_result(&round)))
    .sum();
}

fn part2(input: &Vec<Round>) -> i32 {
  return input.iter()
    .map(|round| {
      let redone = Round { me: matching_choice(&round), them: round.them };
      return shape_score(&redone.me) + round_score(&round_result(&redone));
    })
    .sum();
}

pub fn run() {
  let input = include_str!("data.txt");
  let parsed = parse(input);
  println!("Part 1 {}", part1(&parsed));
  println!("Part 2 {}", part2(&parsed));
}

#[cfg(test)]
mod day1 {
use super::*;

#[test]
fn sample_data() {
  let input = include_str!("sample.txt");
  let parsed = parse(input);
  assert_eq!(part1(&parsed), 24000);
  assert_eq!(part2(&parsed), 45000);
}
}