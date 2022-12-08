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

fn calculate_priorities(compartments: Vec<&str>) -> u32 {
    let set: HashSet<char> = HashSet::from_iter(compartments[0].chars().clone());
    for c in set {
        let mut found = true;
        for i in 1..compartments.len() {
            found = found && compartments[i].contains(c);
        }

        if found {
            return value_of_package(c);
        }
    }
    return 0;
}

pub fn part1() {
    let input = std::fs::read_to_string("day3.txt")
        .expect("There should be a day3.txt file containing puzzle input");

    let solution = input
        .split("\n")
        .map(|x| x.split_at(x.len() / 2))
        .map(|x| vec![x.0, x.1])
        .fold(std::u32::MIN, |total, x| total + calculate_priorities(x));
    println!("Solution to day3/part1: {}", solution);
}

pub fn part2() {
    let input = std::fs::read_to_string("day3.txt")
        .expect("There should be a day3.txt file containing puzzle input");

    let l: Vec<&str> = input.lines().collect();
    println!(
        "Solution to day3/part2: {}",
        l.chunks(3).fold(std::u32::MIN, |total, x| total
            + calculate_priorities(x.to_vec()))
    );
}
