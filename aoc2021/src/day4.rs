fn main() {
    let input: Vec<_> = include_str!("../inputs/day4.txt").split("\n\n").collect();
    let numbers: Vec<i32> = input[0]
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let boards: Vec<Vec<Vec<Option<i32>>>> = input
        .into_iter()
        .skip(1)
        .map(|board| {
            board
                .split('\n')
                .filter_map(|row| {
                    let r: Vec<Option<i32>> = row
                        .trim()
                        .split_ascii_whitespace()
                        .map(|n| Some(n.parse::<i32>().unwrap()))
                        .collect();
                    if r.is_empty() {
                        None
                    } else {
                        Some(r)
                    }
                })
                .collect()
        })
        .collect();
    println!("Day 4, Solution 1: {}", solve1(&numbers, boards.clone()));
    println!("Day 4, Solution 2: {}", solve2(&numbers, boards));
}

fn check_win(board: &[Vec<Option<i32>>]) -> bool {
    let mut found_hor = true;
    let mut found_ver = true;
    for i in 0..board.len() {
        found_hor = true;
        found_ver = true;
        for j in 0..board[0].len() {
            if board[j][i] != None {
                found_ver = false;
            }
            if board[i][j] != None {
                found_hor = false;
            }
        }
        if found_hor || found_ver {
            break;
        }
    }
    found_hor || found_ver
}

fn remove_number(number: i32, board: &[Vec<Option<i32>>]) -> Vec<Vec<Option<i32>>> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|e| if Some(number) == *e { None } else { *e })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn solve1(numbers: &[i32], mut boards: Vec<Vec<Vec<Option<i32>>>>) -> i32 {
    for number in numbers {
        for board in boards.iter_mut() {
            *board = remove_number(*number, board);

            if check_win(board) {
                return board
                    .iter()
                    .map(|row| row.iter().flatten().sum::<i32>())
                    .sum::<i32>()
                    * number;
            }
        }
    }
    unreachable!()
}

fn solve2(numbers: &[i32], mut boards: Vec<Vec<Vec<Option<i32>>>>) -> i32 {
    for number in numbers {
        boards = boards
            .into_iter()
            .filter_map(|mut board| {
                board = remove_number(*number, &board);

                if check_win(&board) {
                    None
                } else {
                    Some(board)
                }
            })
            .collect();
        if boards.len() == 1 {
            return solve1(numbers, boards);
        }
    }
    unreachable!()
}
