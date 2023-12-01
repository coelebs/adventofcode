use env_logger::Env;
use log::{debug, info};
use std::collections::{HashMap, VecDeque};
use std::fs;

fn find_number(val: &str, revers: bool, part2: bool) -> String {
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

        if part2 {
            for (key, value) in replacements.iter() {
                result = result.replace(key, value)
            }
        }

        let clean: String = result.chars().filter(|x| x.is_numeric()).collect();
        if !clean.is_empty() {
            return clean;
        }
    }

    return "0".to_string();
}

fn determine_calibration(val: &str, part2: bool) -> u32 {
    if val.is_empty() {
        return 0;
    }
    let mut numstring = find_number(val, false, part2);
    numstring.push_str(&find_number(val, true, part2));
    debug!("{}", numstring);

    return numstring
        .parse::<u32>()
        .expect("Should be able to parse a number string");
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let input = fs::read_to_string("input1.txt").expect("Should be able to read input");
    let mut result1: u32 = 0;
    let mut result2: u32 = 0;
    for line in input.split("\n") {
        debug!("{}", line);
        result1 += determine_calibration(line, false);
        result2 += determine_calibration(line, true);
    }
    info!("Calibration total = {:?}", result1);
    info!("Calibration total, with written numbers = {:?}", result2);
}
