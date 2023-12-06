use aoclib::aocinit;
use itertools::Itertools;
use log::{debug, info};

fn determine_options(time: u64, distance: u64) -> u64 {
    let mut result = 0;
    for i in 1..time {
        if (i * (time - i)) > distance {
            result += 1;
        }
    }
    return result;
}

fn day6(input: String) {
    let (time, distance): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|x| x.split_once(":").unwrap().1)
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect_tuple()
        .unwrap();
    debug!("{:#?}", time);
    debug!("{:#?}", distance);

    let result1: u64 = time
        .iter()
        .zip(distance.iter())
        .map(|x| determine_options(*x.0, *x.1))
        .product();
    info!("Record beating options: {result1}");

    let fulltime: u64 = time
        .iter()
        .fold("".to_string(), |acc, x| (acc + &x.to_string()))
        .parse::<u64>()
        .unwrap();
    let fulldistance: u64 = distance
        .iter()
        .fold("".to_string(), |acc, x| (acc + &x.to_string()))
        .parse::<u64>()
        .unwrap();
    debug!("determine_options({fulltime}, {fulldistance})");
    let result2 = determine_options(fulltime, fulldistance);

    info!("Record beating option: {result2}");
}

fn main() {
    aocinit(day6);
}
