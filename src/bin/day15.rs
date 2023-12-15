use aoclib::aocinit;
use log::{debug, info};

fn HASH(input: &str) -> u32 {
    let mut current = 0;
    for v in input.chars() {
        current += v as u32;
        current *= 17;
        current %= 256;
    }

    return current;
}

fn day15(input: String) {
    let init_sequence: Vec<&str> = input.split(",").collect::<Vec<&str>>();
    let result1 = init_sequence.iter().fold(0, |acc, x| acc + HASH(x));
    info!("HASH sum result: {result1}");
}

fn main() {
    aocinit(day15);
}
