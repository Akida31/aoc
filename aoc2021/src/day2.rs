fn main() {
    let input: Vec<_> = include_str!("../inputs/day2.txt")
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect();
    println!("Day 2, Solution 1: {}", solve1(&input));
    println!("Day 2, Solution 2: {}", solve2(&input));
}

fn solve1(input: &[Vec<&str>]) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    for line in input {
        let a: usize = line[1].parse().unwrap();
        match line[0] {
            "forward" => pos += a,
            "up" => depth -= a,
            "down" => depth += a,
            _ => unreachable!(),
        }
    }
    pos * depth
}

fn solve2(input: &[Vec<&str>]) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input {
        let a: usize = line[1].parse().unwrap();
        match line[0] {
            "forward" => {
                pos += a;
                depth += aim * a
            }
            "up" => aim -= a,
            "down" => aim += a,
            _ => unreachable!(),
        }
    }
    pos * depth
}
