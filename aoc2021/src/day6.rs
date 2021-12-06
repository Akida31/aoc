fn main() {
    let input: Vec<_> = include_str!("../inputs/day6.txt")
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect();
    println!("Day 6, Solution 1: {}", solve(input.clone(), 80));
    println!("Day 6, Solution 2: {}", solve(input, 256));
}

fn solve(input: Vec<usize>, days: usize) -> usize {
    let mut fish = vec![0; 9];
    for inp in input {
        fish[inp] += 1;
    }
    for _ in 0..days {
        let zeros = fish[0];
        fish.rotate_left(1);
        fish[6] += zeros;
    }
    fish.into_iter().sum()
}

