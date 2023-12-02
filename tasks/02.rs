use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let mut sum:i32 = 0;
    let mut max_possible_values: HashMap<&str, i32> = HashMap::new();
    max_possible_values.insert("red", 12);
    max_possible_values.insert("green", 13);
    max_possible_values.insert("blue", 14);

    let lines = read_lines("src/input.txt");
    for line in lines {
        let index_of_colon = line.find(":").unwrap();
        let game_num = &line[0..index_of_colon + 1];

        let mut should_sum = true;
        let mut min_possible_values: HashMap<&str, i32> = HashMap::new();
        min_possible_values.insert("red", 0);
        min_possible_values.insert("green", 0);
        min_possible_values.insert("blue", 0);

        let line_str = line.replace(game_num, "").replace(" ", "");
        for cubes in line_str.split(";") {
            for colors in cubes.split(",") {
                let re = Regex::new(r"(\d+|\D+)").unwrap();

                // Extract matching parts from the input string
                let parts: Vec<&str> = re.find_iter(colors).map(|m| m.as_str()).collect();

                // Print the resulting parts
                let mut min_possible_value_for_color = 0;
                for part in parts {
                    match min_possible_values.get(part) {
                        Some(&value_min) => {
                            if value_min < min_possible_value_for_color {
                                min_possible_values.insert(part, min_possible_value_for_color);
                            }
                        }
                        None => {
                            min_possible_value_for_color = part.parse::<i32>().unwrap();
                            continue;
                        }
                    }

                    // match max_possible_values.get(part) {
                    //     Some(&value) => {
                    //         if max_possible_value_for_color > value {
                    //             should_sum = false;
                    //         }
                    //         max_possible_value_for_color = 0;
                    //     }
                    //     None => {
                    //         max_possible_value_for_color = part.parse::<i32>().unwrap();
                    //     }
                    //}
                    //println!("{}", part);
                }

                //println!("{colors}");
            }
            //println!("{cubes}");
        }

        let num = game_num.replace("Game ", "").replace(":", "");
        let mut partial_power = 1;
        if should_sum {
            for (key, value) in min_possible_values.iter() {
                partial_power = partial_power * value;
                println!("Key: {}, Value: {}", key, value);
            }
        }
        sum += partial_power;
        println!("{game_num}");
    }
    print!("{}", sum);
}