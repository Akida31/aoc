import datetime
from os import path
import sys
import requests


def fetch_input(year, day):
    with open(path.join(path.dirname(__file__), ".session.txt")) as session_file:
        session = session_file.read().strip()
    resp = requests.get(
            f"https://adventofcode.com/{year}/day/{day}/input",
            cookies={"session": session}
    )
    if not resp.ok:
        return None
    return resp.text.strip()


def edit_cargo(year, day):
    with open(f"aoc{year}/Cargo.toml", "a") as cargo_file:
        cargo_file.write(f"""[[bin]]
name = "{year}-{day}"
path = "src/day{day}.rs"
""")


def create_template(year, day):
    with open(f"aoc{year}/inputs/day{day}.txt", "a") as input_file:
        inp = fetch_input(year, day)
        if inp is None:
            print("could not fetch input")
        else:
            input_file.write(inp)
    p = f"aoc{year}/src/day{day}.rs"
    if path.exists(p):
        print(f"{p} exists already")
        sys.exit(1)
    with open(p, "w") as src_file:
        src_file.write("""fn main() {
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
    if len(sys.argv) < 3:
        day = input("Day: ")
    else:
        day = sys.argv[2]
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
    if len(sys.argv) < 3:
        year = input("Year: ")
    else:
        year = sys.argv[1]
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
    if len(sys.argv) == 2 and sys.argv[1] == ".":
        date = datetime.datetime.now()
        day = date.day
        year = date.year
    else:
        day = get_day()
        year = get_year()
    print(f"Generating: {year}-{day}")
    create_template(year, day)
    edit_cargo(year, day)
