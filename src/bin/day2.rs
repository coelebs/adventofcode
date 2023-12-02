use log::{info, debug};
use env_logger::Env;
use std::fs;
use std::collections::HashMap;

fn count_lines(line: &str) -> (u32, u32) {
    let (game, score) = line.split_once(":").expect("Line should contain a colon");
    let mut gamenr = game.strip_prefix("Game ").expect("Strip game from").parse::<u32>().expect("Parse number from game");

    let mut total_colors: HashMap<&str, u32> = HashMap::new();
    for set in score.split(";") {
        let mut colors: HashMap<&str, u32> = HashMap::new();
        for color in set.split(",") {
            let (num, color) = color.trim().split_once(" ").expect("Should be able to strip color from string");
            let parsed = num.parse::<u32>().expect("Should be able to parse number");
            colors.entry(color).and_modify(|total| *total += parsed).or_insert(parsed);
        }

        for (color, count) in colors.iter() {
            debug!("{}: {}", color, count);
            let max = match *color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => 0,
            };

            if *count > max {
                gamenr = 0;
            }

            total_colors.entry(color).and_modify(|total| if *count > *total { *total = *count }).or_insert(*count);
        }
    }

    return (gamenr, total_colors.values().product());
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let input = fs::read_to_string("input2.txt").expect("Input should be available");

    let mut legal_sum = 0;
    let mut power_sum = 0;
    for line in input.split("\n") {
        debug!("{}", line);
        if !line.is_empty() {
            let (legal, power) = count_lines(line);
            legal_sum += legal;
            power_sum += power;
        }
    }

    info!("Possible games is: {}", legal_sum);
    info!("Sum of power sets is is: {}", power_sum);
}
