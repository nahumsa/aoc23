use regex::Regex;
use crate::utils::read_file;

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
        Err(e) => None,
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

fn consume_results(games_list: Vec<Game>, ) -> i32 {
    let mut index_to_remove: Vec<i32> = vec![];
    for game in &games_list {
        for cube in &game.cubes {
            match cube.color {
                Color::Red => {
                    if cube.quantity > 12 {
                        index_to_remove.push(game.id);
                        break
                    }
                },
                Color::Green => {
                    if cube.quantity > 13 {
                        index_to_remove.push(game.id);
                        break
                    }
                },
                Color::Blue => {
                    if cube.quantity > 14 {
                        index_to_remove.push(game.id);
                        break
                    }
                },
            }
        }
    }
    games_list.iter().map(|cube| cube.id).sum::<i32>() - index_to_remove.iter().sum::<i32>()
}

pub fn second_challenge_a() {
    let file_content = read_file("src/challenge_files/2.txt").unwrap();
    let strings: Vec<&str> = file_content.split("\n").collect();
    let games = parse_results(strings);
    let result = consume_results(games);
    println!(
        "Challenge 2a - sum of ids: {:?}",
        result
    );
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
        let result = consume_results(games);

        assert_eq!(result, 8);

    }
}
