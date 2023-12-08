use aoclib::aocinit;
use log::{debug, info};
use regex::Regex;
use std::collections::HashMap;
use num::Integer;

fn step_to_target(path: &HashMap<&str, (&str, &str)>, cmd: &Vec<char>, start: &str) -> (u64, String) {
    let mut key = start;
    let mut result1: u64 = 0;
    while !key.ends_with("Z") || result1 == 0 {
        let c = cmd[result1 as usize % cmd.len()];
        if c == 'L' {
            key = path[key].0;
        } else if c == 'R' {
            key = path[key].1;
        }
        result1 += 1;
    }

    return (result1, key.to_string());
}

fn day8(input: String) {
    let (cmd_s, path_s) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
    let mut path: HashMap<&str, (&str, &str)> = HashMap::new();

    for m in re.captures_iter(path_s) {
        let val = (m.get(2).unwrap().as_str(), m.get(3).unwrap().as_str());
        path.insert(m.get(1).unwrap().as_str(), val);
    }

    let cmd: Vec<char> = cmd_s.chars().collect();
    let key: &str = "AAA";
    if path.contains_key(key) {
        let result1 = step_to_target(&path, &cmd, key).0;
        info!("Steps to get from AAA to ZZZ: {result1}");
    }

    let mut keys: Vec<(&str, u64, u64)> = path.keys().filter(|x| x.ends_with("A")).map(|x| (*x, 0, 0)).collect();
    for key in &mut keys {
        let first_z = step_to_target(&path, &cmd, key.0);
        debug!("{}", first_z.1);
        let repeat_z = step_to_target(&path, &cmd, &first_z.1);
        key.1 = first_z.0;
        key.2 = repeat_z.0;
        debug!("key: {:?}", key);
    }
    let result2 = keys.iter().fold(0, |acc, num| if acc == 0 { num.1 } else {acc.lcm(&num.1)});
    info!("Steps to get from *A to *Z: {result2}");
}

fn main() {
    aocinit(day8);
}
