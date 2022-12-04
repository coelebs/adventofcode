// Improvement idea, decide winning and losing rock paper scissors once

fn parse_line(line: &str) -> (u32, u32) {
    let (a, b) = line.split_once(" ").expect("Every line should have two letters an a space");
    let score1 = match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };
    let score2 = match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    return (score1, score2);
}

fn calculate_score(parts: (u32, u32)) -> u32 {
    let mut total = parts.1;
    total += match parts {
        (1, 2) => 6,
        (1, 3) => 0,
        (2, 1) => 0,
        (2, 3) => 6,
        (3, 1) => 6,
        (3, 2) => 0,
        _ => 3,
    };
    return total;
}

fn choose_rps(parts: (u32, u32)) -> (u32, u32) {
    let rps = match parts {
        (1, 1) => 3,
        (1, 2) => 1,
        (1, 3) => 2,
        (2, 1) => 1,
        (2, 2) => 2,
        (2, 3) => 3,
        (3, 1) => 2,
        (3, 2) => 3,
        (3, 3) => 1,
        _ => 0,
    };

    return (parts.0, rps);
}

pub fn part1() -> Vec<(u32, u32)> {
    let input = std::fs::read_to_string("day2.txt").expect("There should be a day2.txt file containing puzzle input");
    let scores: Vec<(u32, u32)> = input.split_terminator("\n").map(|line| parse_line(line)).collect();
    println!("day2/part1: {}", scores.iter().fold(std::u32::MIN, |total, sums| total + calculate_score(*sums)));
    return scores;
}

pub fn part2() {
    let scores = part1();
    println!("day2/part2: {}", scores.iter().map(|x| choose_rps(*x)).fold(std::u32::MIN, |total, rps| total + calculate_score(rps)));
}
