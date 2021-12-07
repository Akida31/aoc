fn main() {
    let input: Vec<_> = include_str!("../inputs/day7.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    println!("Day 7, Solution 1: {}", solve1(input.clone()));
    println!("Day 7, Solution 2: {}", solve2(input));
}

fn solve1(input: Vec<isize>) -> isize {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .map(|n| input.iter().map(|x| (x - n).abs()).sum())
        .min()
        .unwrap()
}

fn solve2(input: Vec<isize>) -> isize {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .map(|n| {
            input
                .iter()
                .map(|x| {
                    let y = (x - n).abs();
                    y * (y + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
