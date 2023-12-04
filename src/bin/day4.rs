use log::{debug, info};
use std::collections::HashSet;
use aoclib::aocinit;

fn points_from_line(line: &str) -> u32 {
    debug!("{line}");
    let (winning, own) = line
        .split_once("|")
        .expect("Line is not as format prescribes");

    let win = winning
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap_or(u32::MAX))
        .collect::<HashSet<u32>>();
    return own
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap_or(u32::MIN))
        .filter(|x| win.contains(&x))
        .count()
        .try_into()
        .unwrap();
}

fn day4(input: String) {
    let mut result1: u32 = 0;
    let mut result2: u32 = 0;
    let mut copies = Vec::new();

    for line in input.split("\n") {
        let score = points_from_line(line);

        let cards: u32 = (copies.len() + 1).try_into().unwrap();
        debug!("{cards} copies");
        copies = copies
            .iter()
            .map(|x| *x - 1)
            .filter(|x| *x > 0)
            .collect::<Vec<u32>>();

        if score > 0 {
            result1 += u32::pow(2, score - 1);
            for _ in 0..cards {
                copies.push(score);
            }
        }

        result2 += cards;
    }

    info!("Total points: {result1}");
    info!("Total points with rules: {result2}");
}

fn main() {
    aocinit(day4);
}
