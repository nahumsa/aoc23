use crate::utils::read_file;
use regex::Regex;

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

fn parse_results(strings: Vec<&str>, regex_pattern: &str) -> Vec<i32> {
    let number_pattern = Regex::new(regex_pattern).unwrap();

    let mut calibration_values: Vec<i32> = Vec::new();

    for (_index, s) in strings.iter().enumerate() {
        let replaced_string = s
            .to_string()
            .replace("twone", "twoone")
            .replace("eightwo", "eighttwo")
            .replace("eighthree", "eightthree")
            .replace("oneight", "oneeight")
            .replace("fiveight", "fiveeight")
            .replace("sevenine", "sevennine")
            .replace("nineight", "nineeight")
            .replace("threeight", "threeeight");

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
        }
    }
    calibration_values
}

pub fn first_challenge_a() {
    let file_content = read_file("src/challenge_files/1.txt").unwrap();
    let regex_pattern = r"\d";

    let strings: Vec<&str> = file_content.split("\n").collect();

    let calibration_values = parse_results(strings, regex_pattern);
    println!(
        "Challenge 1a - sum of numbers: {:?}",
        calibration_values.iter().sum::<i32>()
    );
}

pub fn first_challenge_b() {
    let file_content = read_file("src/challenge_files/1.txt").unwrap();
    let regex_pattern = r"(?:\d|one|two|three|four|five|six|seven|eight|nine)";

    let strings: Vec<&str> = file_content.split("\n").collect();

    let calibration_values = parse_results(strings, regex_pattern);
    println!(
        "Challenge 1b - sum of numbers: {:?}",
        calibration_values.iter().sum::<i32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_results() {
        let regex_pattern = r"(?:\d|one|two|three|four|five|six|seven|eight|nine)";
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
        let calibration_values = parse_results(strings, regex_pattern);
        assert_eq!(
            calibration_values,
            vec![12, 38, 15, 77, 14, 13, 11, 99, 29, 83, 13, 24, 42, 14, 76]
        );
    }

    #[test]
    fn test_parse_results_number_literal() {
        let regex_pattern = r"(?:\d|one|two|three|four|five|six|seven|eight|nine)";
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
        let calibration_values = parse_results(strings, regex_pattern);
        assert_eq!(calibration_values, vec![29, 83, 13, 24, 42, 14, 76, 41]);
    }
}
