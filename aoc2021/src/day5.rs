fn main() {
    let input: Vec<_> = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)")
        .unwrap()
        .captures_iter(include_str!("../inputs/day5.txt"))
        .map(|x| {
            (1..=4)
                .map(|i| x[i].parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    println!("Day 5, Solution 1: {}", solve(&input, false));
    println!("Day 5, Solution 2: {}", solve(&input, true));
}

fn solve(input: &[[usize; 4]], diagonal: bool) -> usize {
    let max = input
        .iter()
        .map(|row| (*row).iter().max().unwrap())
        .max()
        .unwrap()
        + 1;
    let mut fields = vec![vec![0; max]; max];
    for [x0, y0, x1, y1] in input {
        if x0 == x1 {
            for row in fields[*y0.min(y1)..=*y0.max(y1)].iter_mut() {
                row[*x0] += 1;
            }
        }
        if y0 == y1 {
            for x in *x0.min(x1)..=*x0.max(x1) {
                fields[*y0][x] += 1;
            }
        }
        if diagonal {
            let x0 = *x0 as isize;
            let y0 = *y0 as isize;
            let xdiff = *x1 as isize - x0;
            let ydiff = *y1 as isize - y0;
            if ydiff.abs() == xdiff.abs() {
                for i in 0..=xdiff.abs() {
                    fields[(y0 + i * ydiff.signum()) as usize]
                        [(x0 + i * xdiff.signum()) as usize] += 1;
                }
            }
        }
    }
    fields
        .iter()
        .map(|row| row.iter().filter(|p| **p > 1).count())
        .sum()
}

