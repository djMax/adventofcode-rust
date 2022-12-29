use std::include_str;

fn parse(input: &str) -> Vec<i32> {
    let mut sums: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|num| {
                    num.parse::<i32>()
                        .unwrap_or_else(|v| panic!("Num is invalid: {}", v))
                })
        })
        .map(|calories| calories.sum::<i32>())
        .collect();
    sums.sort_by(|a, b| b.cmp(a));
    return sums;
}

fn part1(input: &Vec<i32>) -> i32 {
    return input[0];
}

fn part2(input: &Vec<i32>) -> i32 {
    return input[..3].iter().sum();
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
  fn it_works() {
    let input = include_str!("sample.txt");
    let parsed = parse(input);
    assert_eq!(part1(&parsed), 24000);
    assert_eq!(part2(&parsed), 45000);
  }
}