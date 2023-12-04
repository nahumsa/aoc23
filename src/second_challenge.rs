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
                let mut cube_info: Vec<&str> = s.split_whitespace().collect();
                println!("{:?}", cube_info);
                Some(Cube::new(
                    parse_color(cube_info.get(1).unwrap()).unwrap(),
                    parse_int(cube_info.get(0).unwrap()).unwrap(),
                ))
            })
            .collect();

        println!("{:?}", draw);
        game_return.push(Game {
            id: game_id,
            cubes: draw,
        })
    }

    game_return
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
}
