use std::collections::HashMap;

fn main() {
    let input: Vec<_> = include_str!("../inputs/day8.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let input = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            let output = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            (input, output)
        })
        .collect();
    println!("Day 8, Solution 1: {}", solve1(input.clone()));
    println!("Day 8, Solution 2: {}", solve2(input));
}

fn solve1(input: Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    input
        .into_iter()
        .map(|(_, outputs)| {
            outputs
                .into_iter()
                .filter(|output| matches!(output.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

fn solve2(input: Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    input
        .into_iter()
        .map(|(inputs, outputs)| {
            let mut known_digits: HashMap<char, char> = HashMap::new();
            while known_digits.len() < 7 {}

            outputs
                .into_iter()
                .map(|output| {
                    let mut chars = output.chars().map(|c| known_digits[&c]).collect::<Vec<_>>();
                    chars.sort_unstable();
                    match &*String::from_iter(chars) {
                        "abcefg" => '0',
                        "cf" => '1',
                        "acdeg" => '2',
                        "acdfg" => '3',
                        "bcdf" => '4',
                        "abdfg" => '5',
                        "abdefg" => '6',
                        "acf" => '7',
                        "abcdefg" => '8',
                        "abcdfg" => '9',
                        _ => unreachable!(),
                    }
                })
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

