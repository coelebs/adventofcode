use std::collections::HashSet;

fn value_of_package(package: char) -> u32 {
    let value;
    if package.is_uppercase() {
        value = package.to_digit(36).unwrap() - 'A'.to_digit(36).unwrap() + 27;
    } else {
        value = package.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1;
    }
    return value;
}

fn calculate_priorities(compartments: (&str, &str)) -> u32 {
    let set: HashSet<char> = HashSet::from_iter(compartments.0.chars().clone());
    return set
        .iter()
        .filter(|x| compartments.1.contains(**x))
        .fold(std::u32::MIN, |total, x| total + value_of_package(*x));
}

pub fn part1() {
    let input = std::fs::read_to_string("day3.txt")
        .expect("There should be a day3.txt file containing puzzle input");

    let solution = input
        .split("\n")
        .map(|x| x.split_at(x.len() / 2))
        .fold(std::u32::MIN, |total, x| total + calculate_priorities(x));
    println!("Solution to day3/part1: {}", solution);
}
