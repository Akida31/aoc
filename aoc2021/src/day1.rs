fn main() {
    let input: Vec<_> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Day 1, Solution 1: {}", solve1(&input));
    println!("Day 1, Solution 2: {}", solve2(&input));
}

fn solve1(input: &[i32]) -> usize {
    input.windows(2).filter(|x| x[1] > x[0]).count()
}

fn solve2(input: &[i32]) -> usize {
    solve1(&input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>())
}
