use crate::utils::read_file;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}
fn parse_color(input: &str) -> Option<Color> {
    match input.to_lowercase().as_str() {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        _ => None,
    }
}

fn parse_int(input: &str) -> Option<i32> {
    match input.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

#[derive(Debug, PartialEq)]
struct Cube {
    color: Color,
    quantity: i32,
}

impl Cube {
    fn new(color: Color, quantity: i32) -> Self {
        Self { color, quantity }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    cubes: Vec<Cube>,
}

fn parse_results(strings: Vec<&str>) -> Vec<Game> {
    let id_pattern = Regex::new(r"Game (\d+):").unwrap();
    let mut game_return: Vec<Game> = Vec::new();

    for (_index, s) in strings.iter().enumerate() {
        let mut game_id: i32 = 0;
        if let Some(captures) = id_pattern.captures(s) {
            if let Some(number_str) = captures.get(1) {
                if let Ok(number) = number_str.as_str().parse::<i32>() {
                    game_id += number;
                }
            }
        };
        let draw: Vec<Cube> = s
            .split(r":")
            .nth(1)
            .unwrap_or_default()
            .trim()
            .split(r";")
            .flat_map(|s| s.trim().split(","))
            .flat_map(|s| {
                let cube_info: Vec<&str> = s.split_whitespace().collect();
                Some(Cube::new(
                    parse_color(cube_info.get(1).unwrap()).unwrap(),
                    parse_int(cube_info.get(0).unwrap()).unwrap(),
                ))
            })
            .collect();

        game_return.push(Game {
            id: game_id,
            cubes: draw,
        })
    }

    game_return
}

const MIN_RED_A: i32 = 12;
const MIN_GREEN_A: i32 = 13;
const MIN_BLUE_A: i32 = 14;

fn consume_results_a(games_list: Vec<Game>) -> i32 {
    let mut index_to_remove: Vec<i32> = vec![];
    for game in &games_list {
        for cube in &game.cubes {
            match cube.color {
                Color::Red => {
                    if cube.quantity > MIN_RED_A {
                        index_to_remove.push(game.id);
                        break;
                    }
                }
                Color::Green => {
                    if cube.quantity > MIN_GREEN_A {
                        index_to_remove.push(game.id);
                        break;
                    }
                }
                Color::Blue => {
                    if cube.quantity > MIN_BLUE_A {
                        index_to_remove.push(game.id);
                        break;
                    }
                }
            }
        }
    }

    games_list.iter().map(|cube| cube.id).sum::<i32>() - index_to_remove.iter().sum::<i32>()
}

fn consume_results_b(games_list: Vec<Game>) -> i32 {
    let mut result_games: Vec<Game> = vec![];

    for game in &games_list {
        let mut sample_game = Game {
            id: game.id,
            cubes: vec![
                Cube::new(Color::Red, 0),
                Cube::new(Color::Green, 0),
                Cube::new(Color::Blue, 0),
            ],
        };

        for cube in &game.cubes {
            match cube.color {
                Color::Red => {
                    if cube.quantity > sample_game.cubes[0].quantity {
                        sample_game.cubes[0].quantity = cube.quantity;
                    }
                }
                Color::Green => {
                    if cube.quantity > sample_game.cubes[1].quantity {
                        sample_game.cubes[1].quantity = cube.quantity;
                    }
                }
                Color::Blue => {
                    if cube.quantity > sample_game.cubes[2].quantity {
                        sample_game.cubes[2].quantity = cube.quantity;
                    }
                }
            }
        }
        result_games.push(sample_game);
    }
    result_games
        .iter()
        .map(|game| game.cubes.iter().map(|cube| cube.quantity).product::<i32>())
        .sum::<i32>()
}

pub fn second_challenge_a() {
    let file_content = read_file("src/challenge_files/2.txt").unwrap();
    let strings: Vec<&str> = file_content.split("\n").collect();
    let games = parse_results(strings);
    let result = consume_results_a(games);
    println!("Challenge 2a - sum of ids: {:?}", result);
}

pub fn second_challenge_b() {
    let file_content = read_file("src/challenge_files/2.txt").unwrap();
    let strings: Vec<&str> = file_content.split("\n").collect();
    let games = parse_results(strings);
    let result = consume_results_b(games);
    println!("Challenge 2b - power of games: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2a_parse_results() {
        let strings = vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"];
        let game = parse_results(strings);
        assert_eq!(
            game,
            vec!(Game {
                id: 1,
                cubes: vec![
                    Cube::new(Color::Blue, 3),
                    Cube::new(Color::Red, 4),
                    Cube::new(Color::Red, 1),
                    Cube::new(Color::Green, 2),
                    Cube::new(Color::Blue, 6),
                    Cube::new(Color::Green, 2),
                ],
            })
        );
    }

    #[test]
    fn test_2a_results() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let strings: Vec<&str> = example.split("\n").collect();
        let games = parse_results(strings);
        let result = consume_results_a(games);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_2b_results() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let strings: Vec<&str> = example.split("\n").collect();
        let games = parse_results(strings);
        let result = consume_results_b(games);

        assert_eq!(result, 2286);
    }
}
