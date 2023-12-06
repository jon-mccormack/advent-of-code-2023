use std::fs;

const INPUT_PATH: &str = "/workspaces/advent-of-code-2023/day2/src/input";

#[derive(Clone, Copy)]
struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    hand_1: Hand,
    hand_2: Hand,
    hand_3: Hand,
}

fn get_hand(contents: &str) -> Hand {
    let cubes_split = contents.split(",");

    const RED: &str = "red";
    const BLUE: &str = "blue";
    const GREEN: &str = "green";

    let mut hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };

    for mut cubes in cubes_split {
        cubes = cubes.trim();

        if cubes.contains(RED) {
            hand.red = cubes[0..cubes.find(RED).unwrap() - 1].parse().unwrap();
            continue;
        }
        if cubes.contains(BLUE) {
            hand.blue = cubes[0..cubes.find(BLUE).unwrap() - 1].parse().unwrap();
            continue;
        }
        if cubes.contains(GREEN) {
            hand.green = cubes[0..cubes.find(GREEN).unwrap() - 1].parse().unwrap();
            continue;
        }
    }

    return hand;
}

fn get_games(contents: String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::from([]);

    for line in contents.lines() {
        let colon_pos = line.find(":").unwrap();
        let game_id: i32 = line[5..colon_pos].parse().unwrap();
        let mut hands: Vec<Hand> = Vec::from([]);

        let hands_split = line.clone()[colon_pos + 1..].split(";");
        for hand_content in hands_split.into_iter() {
            hands.push(get_hand(hand_content));
        }

        games.push(Game {
            id: game_id,
            hand_1: hands[0],
            hand_2: hands[1],
            hand_3: hands[2],
        });
    }

    return games;
}

fn part_one() -> i32 {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let games = get_games(contents);

    return 0;
}

fn main() {
    println!("Day one\n\tPart one: {}", part_one());
}
