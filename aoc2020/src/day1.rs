fn main() {
    let input: Vec<_> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Day 1, Solution 1: {}", solve1(input.clone()));
    println!("Day 1, Solution 2: {}", solve2(input));
}

fn solve1(input: Vec<usize>) -> usize {
    for i in &input {
        for j in &input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    unreachable!("no working numbers")
}

fn solve2(input: Vec<usize>) -> usize {
    for i in &input {
        for k in &input {
            for j in &input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!("no working numbers")
}
