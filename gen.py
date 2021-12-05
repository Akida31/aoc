import datetime
from os import path
from sys import argv


def edit_cargo(year, day):
    with open(f"aoc{year}/Cargo.toml", "a") as f:
        f.write(f"""[[bin]]
name = "{year}-{day}"
path = "src/day{day}.rs"
""")


def create_template(year, day):
    open(f"aoc{year}/inputs/day{day}.txt", "a").close()
    p = f"aoc{year}/src/day{day}.rs"
    if path.exists(p):
        quit(f"{p} exists already")
    with open(p, "w") as f:
        f.write("""fn main() {
    let input: Vec<_> = include_str!("../inputs/day@day@.txt").lines().collect();
    println!("Day @day@, Solution 1: {}", solve1(input.clone()));
    println!("Day @day@, Solution 2: {}", solve2(input));
}

fn solve1(input: Vec<&str>) -> usize {
    unimplemented!()
}

fn solve2(input: Vec<&str>) -> usize {
    unimplemented!()
}""".replace("@day@", str(day)))


def get_day():
    if len(argv) < 3:
        day = input("Day: ")
    else:
        day = argv[2]
    try:
        day = int(day)
    except ValueError:
        print("not a valid day [1..25]")
        day = None
    while not day:
        try:
            day = int(input("Day: "))
        except ValueError:
            print("not a valid day [1..25]")
            day = None
    return day

def get_year():
    if len(argv) < 3:
        year = input("Year: ")
    else:
        year = argv[1]
    try:
        year = int(year)
    except ValueError:
        print("not a valid year")
        year = None
    while not year:
        try:
            year = int(input("Year: "))
        except ValueError:
            print("not a valid year")
            year = None
    return year

if __name__ == "__main__":
    if len(argv) == 2 and argv[1] == ".":
        date = datetime.datetime.now()
        day = date.day
        year = date.year
    else:
        day = get_day()
        year = get_year()
    print(f"Generating: {year}-{day}")
    create_template(year, day)
    edit_cargo(year, day)
