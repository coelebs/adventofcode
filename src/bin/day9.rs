use aoclib::aocinit;
use log::{debug, info};

fn determine_next(history: &Vec<i64>) -> i64 {
    let mut history_diff: Vec<i64> = vec![];
    for i in 1..history.len() {
        history_diff.push(history[i] - history[i - 1]);
    }
    let mut next_new = 0;
    if history_diff.iter().filter(|x| **x != 0).count() != 0 {
        next_new = determine_next(&history_diff);
    }

    debug!(
        "Extrapolated value: {}",
        next_new + history_diff.last().unwrap() + history.last().unwrap()
    );
    return next_new + history_diff.last().unwrap();
}

fn determine_prev(history: &Vec<i64>) -> i64 {
    let mut history_diff: Vec<i64> = vec![];
    for i in 1..history.len() {
        history_diff.push(history[i] - history[i - 1]);
    }
    let mut next_new = 0;
    if history_diff.iter().filter(|x| **x != 0).count() != 0 {
        next_new = determine_prev(&history_diff);
    }

    debug!(
        "Extrapolated value: {}",
        history.first().unwrap() - (history_diff.first().unwrap() - next_new)
    );
    return history_diff.first().unwrap() - next_new;
}

fn day9(input: String) {
    let history: Vec<Vec<i64>> = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let result1 = history.iter().fold(0, |acc, x| {
        debug!("----");
        acc + determine_next(x) + x.last().unwrap()
    });
    info!("Extrapolated sum: {result1}");

    let result2 = history.iter().fold(0, |acc, x| {
        debug!("----");
        acc + (x.first().unwrap() - determine_prev(x))
    });
    info!("Extrapolated sum: {result2}");
}

fn main() {
    aocinit(day9);
}
