
const MAX_RED: i32 = 12;
const MAX_BLUE: i32 = 14;
const MAX_GREEN: i32 = 13;

fn parse_game(part: &str) -> [i32; 3] {
    let mut red: i32 = 0;
    let mut blue: i32 = 0;
    let mut green: i32 = 0;
    let part = part.trim();
    let part_parts = part.split(",").collect::<Vec<&str>>();
    for part in part_parts {
        let color = part.trim().split(" ").collect::<Vec<&str>>();
        let count = color[0].parse::<i32>().unwrap();
        match color[1] {
            "red" => { red = count },
            "blue" => { blue = count },
            "green" => {green = count },
            _ => panic!("Invalid color: {:?}", color),
        }
    }
    [red, blue, green]
}

fn validate_game(game: &str) -> bool {
    let game = game.split(":").collect::<Vec<&str>>();
    let game_parts = game[1].split(";").collect::<Vec<&str>>();
    for part in game_parts {
        let parsed = parse_game(part);
        let red = parsed[0];
        let blue = parsed[1];
        let green = parsed[2];
        if red > MAX_RED || blue > MAX_BLUE || green > MAX_GREEN {
            return false;
        }
    }
    true
}

fn main() {
    let games = include_str!("../input").lines();
    let mut part1_value = 0;
    for game in games {
        if validate_game(game) {
            let game_number = game.split(": ").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            part1_value += game_number;
        }
    }
    println!("Valid games: {}", part1_value);
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_day2_part1_valid_game() {
        let mut games = HashMap::new();
        games.insert(1, HashMap::from([("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]));
        games.insert(2, HashMap::from([("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", true)]));
        games.insert(3, HashMap::from([("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false)]));
        games.insert(4, HashMap::from([("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", false)]));
        games.insert(5, HashMap::from([("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]));

        for (_, value) in games {
            let game = value.keys().next().unwrap();
            let expected = value.values().next().unwrap();
            assert_eq!(validate_game(game), *expected);
        }

    }
}