use std::{collections::HashMap, fs};

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

fn find_last(line: &str, item: &str) -> Option<usize> {
    let mut found_idx = None;
    let mut search_line = line.clone();
    let mut chars_removed = 0;

    loop {
        let idx = search_line.find(item);
        if idx.is_none() && found_idx.is_none() {
            return None;
        }

        if idx.is_none() {
            return Some(chars_removed - 1);
        }

        chars_removed = chars_removed + idx.unwrap() + 1;
        found_idx = idx;
        search_line = &search_line[idx.unwrap() + 1..];
    }
}

#[test]
fn test_find_last() {
    assert_eq!(find_last("123", "4"), None);
    assert_eq!(find_last("123", "1"), Some(0));
    assert_eq!(find_last("111", "1"), Some(2));
    assert_eq!(find_last("1one1one1oneone", "one"), Some(12));
    assert_eq!(find_last("onetwothreefourthreetwoone", "three"), Some(15));
}

fn part_two() -> u32 {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut total: u32 = 0;

    let mut numbers_map: HashMap<&str, i32> = HashMap::new();
    numbers_map.insert("one", 1);
    numbers_map.insert("two", 2);
    numbers_map.insert("three", 3);
    numbers_map.insert("four", 4);
    numbers_map.insert("five", 5);
    numbers_map.insert("six", 6);
    numbers_map.insert("seven", 7);
    numbers_map.insert("eight", 8);
    numbers_map.insert("nine", 9);
    numbers_map.insert("1", 1);
    numbers_map.insert("2", 2);
    numbers_map.insert("3", 3);
    numbers_map.insert("4", 4);
    numbers_map.insert("5", 5);
    numbers_map.insert("6", 6);
    numbers_map.insert("7", 7);
    numbers_map.insert("8", 8);
    numbers_map.insert("9", 9);

    let numbers = Vec::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ]);

    for line in contents.lines() {
        // firsts and lasts contain the index of the `numbers` HashMap where the number was found first and last, respectively
        let firsts: Vec<Option<usize>> = numbers.clone().iter().map(|x| line.find(x)).collect();
        let lasts: Vec<Option<usize>> =
            numbers.clone().iter().map(|x| find_last(line, x)).collect();

        // find the lowest value (that is not None) in firsts
        let mut first_key: Option<&str> = None;
        let mut first_key_pos: Option<usize> = None;
        let mut first_key_val: Option<usize> = None;
        for (idx, elem) in firsts.iter().enumerate() {
            if first_key.is_none() && elem.is_some() {
                first_key = Some(numbers[idx]);
                first_key_pos = Some(idx);
                first_key_val = *elem;
                continue;
            }

            if elem.is_some() && elem.unwrap() < first_key_val.unwrap() {
                first_key = Some(numbers[idx]);
                first_key_pos = Some(idx);
                first_key_val = *elem;
            }
        }

        let mut last_key: Option<&str> = None;
        let mut last_key_pos: Option<usize> = None;
        let mut last_key_val: Option<usize> = None;
        for (idx, elem) in lasts.iter().enumerate() {
            if last_key.is_none() && elem.is_some() {
                last_key = Some(numbers[idx]);
                last_key_pos = Some(idx);
                last_key_val = *elem;
                continue;
            }

            if elem.is_some() && elem.unwrap() > last_key_val.unwrap() {
                last_key = Some(numbers[idx]);
                last_key_pos = Some(idx);
                last_key_val = *elem;
            }
        }

        let num = (numbers_map
            .get_key_value(numbers[first_key_pos.unwrap()])
            .unwrap()
            .1
            * 10)
            + numbers_map
                .get_key_value(numbers[last_key_pos.unwrap()])
                .unwrap()
                .1;
        total = total + num as u32;
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
