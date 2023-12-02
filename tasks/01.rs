use aho_corasick::AhoCorasick;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_numbers(input: &str) -> i32 {

    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replace_with = &["1ne", "2wo", "3hree", "4our", "5ive", "6ix", "7even", "8ight", "9ine"];

    let ac = AhoCorasick::new(patterns);
    let result_str = ac
        .unwrap()
        .replace_all(input, replace_with);
    let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let ac = AhoCorasick::new(patterns);
    let result_char_vec:Vec<char> = ac
        .unwrap()
        .replace_all(&result_str, replace_with)
        .chars()
        .filter(|c| c.is_numeric())
        .collect();

    let mut result:i32 = 0;
    if let Some(first_item) = result_char_vec.first() {
        result = first_item.to_digit(10).unwrap() as i32;
    }

    if let Some(last_item) = result_char_vec.last() {
        result = result * 10 + last_item.to_digit(10).unwrap() as i32;
    }

    return result;
}

fn main() {
    let mut sum:i32 = 0;

    let lines = read_lines("src/input.txt");
    for line in lines {
        let part = get_numbers(&line);
        sum += part;

        println!("line: {} value: {}", line, part);
    }
    print!("{}", sum);
}