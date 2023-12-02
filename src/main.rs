use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::io::Result;

fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn word_to_number(word: &str) -> Option<i32> {
    match word.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_results(strings: Vec<&str>) -> Vec<i32> {
    let number_pattern =
        Regex::new(r"(?:\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut calibration_values: Vec<i32> = Vec::new();

    for (index, s) in strings.iter().enumerate() {
        let replaced_string = s.to_string().replace("twone", "twoone");
        let matches: Vec<&str> = number_pattern
            .find_iter(replaced_string.as_str())
            .map(|m| m.as_str())
            .collect();
        if matches.len() == 1 {
            if let Ok(first_digit) = matches[0].parse::<i32>() {
                let result_digit = first_digit * 10 + first_digit;
                calibration_values.push(result_digit);
            } else {
                match word_to_number(&matches[0].to_lowercase()) {
                    Some(first_digit) => {
                        let result_digit = first_digit * 10 + first_digit;
                        calibration_values.push(result_digit);
                    }
                    None => println!("Unknown word: {}", matches[0]),
                }
            }
        } else if matches.len() == 0 {
            {}
        } else {
            // Initialize variables to hold the first and last digits
            let mut first_digit: i32 = 0;
            let mut last_digit: i32 = 0;

            // Convert the first digit
            if let Ok(digit) = matches[0].parse::<i32>() {
                first_digit += digit;
            } else {
                match word_to_number(&matches[0].to_lowercase()) {
                    Some(digit) => first_digit += digit,
                    None => {
                        println!("Unknown word: {}", matches[0]);
                    }
                }
            }

            // Convert the last digit
            if let Ok(digit) = matches[matches.len() - 1].parse::<i32>() {
                last_digit += digit;
            } else {
                match word_to_number(&matches[matches.len() - 1].to_lowercase()) {
                    Some(digit) => last_digit += digit,
                    None => {
                        println!("Unknown word: {}", matches[matches.len() - 1]);
                    }
                }
            }

            let result_digit = first_digit * 10 + last_digit;
            calibration_values.push(result_digit);

            // println!("{:?}", s);
            // println!(
            //     "{:?} matches: {:?} result {:?}",
            //     index, matches, result_digit
            // );
        }
    }
    println!("{:?}", calibration_values);
    calibration_values
}

fn first_challenge() {
    let file_content = read_file("src/challenge_files/1.txt").unwrap();

    let strings: Vec<&str> = file_content.split("\n").collect();

    let calibration_values = parse_results(strings);
    println!(
        "sum of numbers: {:?}",
        calibration_values.iter().sum::<i32>()
    );
}

fn main() {
    first_challenge()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_results() {
        let strings = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
            "tre123ab4",
            "oneprthree",
            "one",
            "nine",
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let calibration_values = parse_results(strings);
        assert_eq!(
            calibration_values,
            vec![12, 38, 15, 77, 14, 13, 11, 99, 29, 83, 13, 24, 42, 14, 76]
        );
    }

    #[test]
    fn test_parse_results_number_literal() {
        let strings = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "trknlxnv43zxlrqjtwonect",
        ];
        let calibration_values = parse_results(strings);
        assert_eq!(calibration_values, vec![29, 83, 13, 24, 42, 14, 76, 41]);
    }
}
