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

fn parse_results(strings: Vec<&str>) -> Vec<i32> {
    let number_pattern = Regex::new(r"\d").unwrap();
    let mut calibration_values: Vec<i32> = Vec::new();

    for (index, s) in strings.iter().enumerate() {
        let matches: Vec<&str> = number_pattern.find_iter(s).map(|m| m.as_str()).collect();
        if matches.len() == 1 {
            if let Ok(first_digit) = matches[0].parse::<i32>() {
                let result_digit = first_digit * 10 + first_digit;
                calibration_values.push(result_digit);
            } else {
                eprintln!("Error parsing the first digit in '{}'", s);
            }
        } else if matches.len() == 0 {
            {}
        } else {
            println!("{:?} matches: {:?}", index, matches);
            if let (Ok(first_digit), Ok(last_digit)) = (
                matches[0].parse::<i32>(),
                matches[matches.len() - 1].parse::<i32>(),
            ) {
                let result_digit = first_digit * 10 + last_digit;
                calibration_values.push(result_digit);
            } else {
                eprintln!("Error parsing digits in '{}'", s);
            }
        }
    }
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
        ];
        let calibration_values = parse_results(strings);
        assert_eq!(calibration_values, vec![12, 38, 15, 77, 14]);
    }
}
