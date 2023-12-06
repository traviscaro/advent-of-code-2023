use std::{fs::File, io, io::BufRead};

/**
     --- Day 1: Trebuchet?! ---
    Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

    You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

    You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

    As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

    The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

    For example:

    1abc2invi
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

    Consider your entire calibration document. What is the sum of all of the calibration values?

    --- Part Two ---
    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

    Equipped with this new information, you now need to find the real first and last digit on each line. For example:

    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

    What is the sum of all of the calibration values?
*/
fn main() {
    let calibration_sum = sum_calibration_values(CalibrationOptions {
        file_path: "./input.txt".to_string(),
        convert_num_strings: false,
    });
    println!("Part 1 Answer: {}", calibration_sum);

    let adjusted_calibration_sum = sum_calibration_values(CalibrationOptions {
        file_path: "./input.txt".to_string(),
        convert_num_strings: true,
    });
    println!("Part 2 Answer: {}", adjusted_calibration_sum);
}

struct CalibrationOptions {
    file_path: String,
    convert_num_strings: bool,
}

fn sum_calibration_values(options: CalibrationOptions) -> i32 {
    let calibration_inputs = load_calibration_inputs_from_file(options.file_path);

    let calibration_values: Vec<i32>;
    if options.convert_num_strings {
        let converted_calibration_inputs = calibration_inputs
            .iter()
            .map(|calibration_input| convert_input_num_strings(calibration_input.to_string()))
            .collect();

        calibration_values = parse_calibration_values(converted_calibration_inputs);
    } else {
        calibration_values = parse_calibration_values(calibration_inputs);
    }

    return calibration_values.iter().sum();
}

// Load the calibration inputs from the file
fn load_calibration_inputs_from_file(file_path: String) -> Vec<String> {
    // Open the file
    let file = File::open(&file_path).unwrap();
    let reader = io::BufReader::new(file);

    // Read the file line by line into a vector of strings
    // let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    return lines;
}

fn parse_calibration_values(calibration_inputs: Vec<String>) -> Vec<i32> {
    return calibration_inputs
        .iter()
        .map(|calibration_input| parse_calibration_value(&calibration_input))
        .collect();
}

// Parse the calibration input to the calibration value by combining the first
// digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//  1abc2
//  pqr3stu8vwx
//  a1b2c3d4e5f
//  treb7uchet
//
// In this example, the calibration values of these four lines are 12, 38, 15, and 77.
// Adding these together produces 142.
fn parse_calibration_value(calibration_input: &str) -> i32 {
    let first_digit = calibration_input
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let last_digit = calibration_input
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let calibration_value_str = format!("{}{}", first_digit, last_digit);
    let calibration_value: i32 = calibration_value_str.parse().unwrap();

    return calibration_value;
}

// Replace the spelled out numbers with their numeric values
// Note: A scenario exists where the spelled out numbers if replaced will create an issue.
// For example, let's consider the string "xtwone3four".
// If we just string replace in order, "one" will be replaced with "1" and then "two" will not
// detected because it becomes "tw1". This issue also occurs with "nine" and "seven" because of
// the "n". same for "three", "nine", and "eight" because of the "e".
fn convert_input_num_strings(calibration_input: String) -> String {
    return calibration_input
        .replace("nine", "n9e")
        .replace("eight", "e8t")
        .replace("seven", "s7n")
        .replace("six", "s6x")
        .replace("five", "f5e")
        .replace("four", "f4r")
        .replace("three", "t3e")
        .replace("two", "t2o")
        .replace("one", "o1e");
}

#[cfg(test)]
mod tests {
    use crate::{convert_input_num_strings, parse_calibration_value, parse_calibration_values};

    #[test]
    fn test_parse_calibration_value() {
        assert_eq!(parse_calibration_value("1abc2"), 12);
    }

    #[test]
    fn test_parse_calibration_values() {
        let inputs = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        assert_eq!(parse_calibration_values(inputs), [12, 38, 15, 77]);
    }

    #[test]
    fn test_convert_input_num_strings() {
        assert_eq!(
            convert_input_num_strings("7pqrstsixteen".to_string()),
            "7pqrst6teen"
        );
    }
}
