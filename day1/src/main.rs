use std::{fs, cmp::Ordering};

const INPUT_PATH: &str = "/workspaces/advent-of-code-2023/day1/src/input";

fn part_one() -> u32 {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut total = 0;

    for line in contents.lines() {
        let num_chars: Vec<u32> = line
            .chars()
            .filter(|&x| x.is_numeric())
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let first = num_chars.first().unwrap();
        let last = num_chars.last().unwrap();
        let num = (first * 10) + last;
        total = total + num;
    }

    return total;
}

fn cmp_optional_num(a: &Option<usize>, b: &Option<usize>) -> Ordering {
    if a.is_some() && b.is_none() {
        return Ordering::Greater;
    }
    if a.is_none() && b.is_some() {
        return Ordering::Less;
    }
    if a.is_none() && b.is_none() {
        return Ordering::Equal;
    }

    return a.unwrap().cmp(&b.unwrap());
}

fn part_two() -> u32 {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut total = 0;

    let numbers = Vec::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ]);

    for line in contents.lines() {
        let mut firsts: Vec<Option<usize>> = numbers.clone().iter().map(|&x| line.find(x)).collect();
        // find the lowest of firsts
        firsts.sort_by(cmp_optional_num);

    }

    return total;
}

fn main() {
    println!(
        "Day one\n\tPart one: {}\n\tPart two: {}",
        part_one(),
        part_two()
    );
}
