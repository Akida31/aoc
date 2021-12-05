fn main() {
    let input: Vec<_> = include_str!("../inputs/day3.txt").lines().collect();
    println!("Day 3, Solution 1: {}", solve1(&input));
    println!("Day 3, Solution 2: {}", solve2(&input));
}
fn solve1(input: &[&str]) -> u32 {
    let (gamma, epsilon) =
        (0..input[0].chars().count())
            .into_iter()
            .fold((0, 0), |(mut gamma, mut epsilon), i| {
                gamma <<= 1;
                epsilon <<= 1;
                let ones = input
                    .iter()
                    .map(|line| line.chars().nth(i).unwrap())
                    .filter(|x| *x == '1')
                    .count();
                if ones > input.len() / 2 {
                    gamma += 1;
                } else {
                    epsilon += 1;
                };
                (gamma, epsilon)
            });
    gamma * epsilon
}

fn solve2(input: &[&str]) -> isize {
    let mut i = 0;
    let mut oxygen = Vec::from(input);
    while oxygen.len() > 1 {
        let ones = oxygen
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|x| *x == '1')
            .count();
        let more = if ones as f32 >= oxygen.len() as f32 / 2. {
            '1'
        } else {
            '0'
        };
        oxygen = oxygen
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap_or('x') == more)
            .collect();
        i += 1;
    }
    let mut c02 = Vec::from(input);
    i = 0;
    while c02.len() > 1 {
        let ones = c02
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|x| *x == '1')
            .count();
        let fewer = if ones as f32 >= c02.len() as f32 / 2. {
            '0'
        } else {
            '1'
        };
        c02 = c02
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap_or('x') == fewer)
            .collect();
        i += 1;
    }

    let oxygen = isize::from_str_radix(oxygen[0], 2).unwrap();
    let c02 = isize::from_str_radix(c02[0], 2).unwrap();
    oxygen * c02
}
