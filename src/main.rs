use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn find_number(val: &str, revers: bool) -> String {
    let mut letters: VecDeque<char> = val.chars().collect();
    let replacements = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut result = String::new();
    while !letters.is_empty() {
        if revers {
            result.insert(0, letters.pop_back().expect("Should be a letter to pop"));
        } else {
            result.push(letters.pop_front().expect("Should be a letter to pop"));
        }

        for (key, value) in replacements.iter() {
            result = result.replace(key, value)
        }

        let clean: String = result.chars().filter(|x| x.is_numeric()).collect();
        if !clean.is_empty() {
            return clean;
        }
    }

    return "0".to_string();
}

fn determine_calibration(val: &str) -> u32 {
    if val.is_empty() {
        return 0;
    }
    let mut numstring = find_number(val, false);
    numstring.push_str(&find_number(val, true));
    println!("{}", numstring);

    return numstring
        .parse::<u32>()
        .expect("Should be able to parse a number string");
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Should be able to read input");
    let mut result: u32 = 0;
    for line in input.split("\n") {
        println!("{}", line);
        result += determine_calibration(line);
    }
    println!("Calibration total = {:?}", result);
}
