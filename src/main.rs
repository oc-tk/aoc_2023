use aho_corasick::AhoCorasick;
use std::fs::read_to_string;
use std::ptr::replace;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

// fn get_numbers(input: &str) -> i32 {
//     let result_vec:Vec<char> = input
//         .chars()
//         .filter(|c| c.is_numeric()).collect();
//
//     let mut result:i32 = 0;
//     if let Some(first_item) = result_vec.first() {
//         result = first_item.to_digit(10).unwrap() as i32;
//     }
//
//     if let Some(last_item) = result_vec.last() {
//         result = result * 10 + last_item.to_digit(10).unwrap() as i32;;
//     }
//
//     return result;
// }

fn get_numbers(input: &str) -> i32 {
    // let mapping: Vec<(&str, &str)> = vec![
    //     ("one", "1ne"),
    //     ("two", "2wo"),
    //     ("three", "3hree"),
    //     ("four", "4our"),
    //     ("five", "5ive"),
    //     ("six", "6ix"),
    //     ("seven", "7even"),
    //     ("eight", "8ight"),
    //     ("nine", "9ine"),
    //     ("1", "1"),
    //     ("2", "2"),
    //     ("3", "3"),
    //     ("4", "4"),
    //     ("5", "5"),
    //     ("6", "6"),
    //     ("7", "7"),
    //     ("8", "8"),
    //     ("9", "9"),
    //     ("0", "10")
    // ];

    if input == "1six15ninebgnzhtbmlxpnrqoneightfhp" {
        println!("yey");
    }

    // let mut result_vec = Vec::new();
    // let mut start_index = 0;
    //
    //
    // while start_index < input.len() {
    //     // Find the next numeric substring
    //     let (_, remaining, rem_index):(&str, &str, usize) = mapping.iter().find_map(|&(s, value)| {
    //         if input[start_index..].starts_with(s) {
    //             Some((s, value, start_index + s.len()))
    //         } else {
    //             None
    //         }
    //     }).unwrap_or(("other", "", start_index + 1)); // Default case for non-matching substring
    //
    //     // Update start_index to the next position
    //     start_index = rem_index;
    //
    //     // Add the numeric value to the result
    //     result_vec.push(remaining);
    // }

    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replace_with = &["1ne", "2wo", "3hree", "4our", "5ive", "6ix", "7even", "8ight", "9ine"];

    let ac = AhoCorasick::new(patterns);
    let result_str = ac.unwrap().replace_all(input, replace_with);
    let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let ac = AhoCorasick::new(patterns);
    let result_char_vec:Vec<char> = ac.unwrap().replace_all(&result_str, replace_with).chars().filter(|c| c.is_numeric()).collect();

    let mut result:i32 = 0;
    if let Some(first_item) = result_char_vec.first() {
        result = first_item.to_digit(10).unwrap() as i32;
    }

    if let Some(last_item) = result_char_vec.last() {
        result = result * 10 + last_item.to_digit(10).unwrap() as i32;;
    }

    // let mut result:i32 = 0;
    // if let Some(first_item) = result_vec.first() {
    //     result = *first_item
    // }
    //
    // if let Some(last_item) = result_vec.last() {
    //     result = result * 10 + *last_item;
    // }
    // else {
    //     result = result * 10 + result
    // }

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