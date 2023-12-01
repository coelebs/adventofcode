use std::fs;

fn determine_calibration(val: &str) -> u32 {
    if val.is_empty() { return 0; }
    let mut numstr = String::new();
    let clean: String = val.chars().filter(|x| x.is_numeric()).collect();
    numstr.push(clean.chars().next().expect("Should be able to get first char from string"));
    numstr.push(clean.chars().last().expect("Should be able to get last char from string"));
    return numstr.parse::<u32>().expect("Should be able to parse an int");
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Should be able to read input");
    let mut result: u32 = 0;
    for line in input.split("\n") {
        result += determine_calibration(line);
    }
    println!("Calibration total = {:?}", result);
}
