use std::env;
use env_logger::Env;
use log::{info, debug, error};
use std::fs;

fn part_number_indication(dut: char) -> bool {
    return !dut.is_whitespace() && !dut.is_digit(10) && dut != '.';
}

fn determine_part_number(index: (usize, usize), len: usize, matrix: &Vec<Vec<char>>) -> bool {
    let start = if index.1 == 0 { 0 } else { index.1 - 1 };
    let end = if index.1 + len == matrix[0].len() { matrix[0].len() } else { index.1 + len + 1 };
    for j in start..end {
        if index.0 > 0 {
            if part_number_indication(matrix[index.0 - 1][j]) {
                return true;
            }
        }

        if index.0 < matrix.len() - 1 {
            if part_number_indication(matrix[index.0 + 1][j]) {
                return true;
            }
        }

        if (index.1 > 0 && j == index.1 - 1) || (index.1 + len < index.1 + matrix[0].len() && j == index.1 + len) {
            if part_number_indication(matrix[index.0][j]) {
                return true;
            }
        }
    }

    return false;
}

fn determine_gear_ratio(index: (usize, usize), len: usize, matrix: &Vec<Vec<char>>) -> u32 {
    let mut firstresult = 0;
    let mut secondresult = 0;

    let start = if index.1 == 0 { 0 } else { index.1 - 1 };
    let end = if index.1 + len == matrix[0].len() { matrix[0].len() } else { index.1 + len + 1 };

    if index.0 > 0 {
        for j in start..end {
            if matrix[index.0 - 1][j].is_digit(10) {
                let num = num_len(((index.0 - 1), j), matrix);
                if firstresult == 0 {
                    firstresult = num;
                } else if secondresult == 0 {
                    secondresult = num;
                } else {
                    return 0;
                }

                if matrix[index.0 - 1][j + 1] != '.' {
                    break;
                }
            }
        }
    }

    if index.0 < matrix.len() - 1 {
        for j in start..end {
            if matrix[index.0 + 1][j].is_digit(10) {
                let num = num_len(((index.0 + 1), j), matrix);
                if firstresult == 0 {
                    firstresult = num;
                } else if secondresult == 0 {
                    secondresult = num;
                } else {
                    return 0;
                }

                if matrix[index.0 + 1][j + 1] != '.' {
                    break;
                }
            }
        }
    }

    if index.1 > 0 && matrix[index.0][index.1 - 1].is_digit(10) {
        let num = num_len((index.0, index.1 - 1), matrix);
        if firstresult == 0 {
            firstresult = num;
        } else if secondresult == 0 {
            secondresult = num;
        } else {
            return 0;
        }
    }

    if index.1 < matrix[index.0].len() && matrix[index.0][index.1 + 1].is_digit(10) {
        let num = num_len((index.0, index.1 + 1), matrix);
        if firstresult == 0 {
            firstresult = num;
        } else if secondresult == 0 {
            secondresult = num;
        } else {
            return 0;
        }
    }

    if firstresult != 0 && secondresult != 0 {
        debug!("ratio: {firstresult} * {secondresult}");
    }
    return firstresult * secondresult;
}

fn num_len(index: (usize, usize), matrix: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    let mut i = index.1;
    while i < matrix[0].len() && matrix[index.0][i].is_digit(10) {
        result *= 10;
        result += matrix[index.0][i].to_digit(10).expect("Should be a number");
        i += 1;
    }

    debug!("i is {i}");
    let mut num_len = (i - index.1).try_into().unwrap();
    if index.1 > 0 {
        i = index.1 - 1;
        while matrix[index.0][i].is_digit(10) {
            result += matrix[index.0][i].to_digit(10).expect("Should be a number") * u32::pow(10, num_len);
            num_len += 1;
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
    }

    return result;
}

fn day3(input: String) {
    let matrix: Vec<Vec<char>> = input.trim().split("\n").map(|line| line.chars().collect()).collect();
    let mut result1: u32 = 0;
    let mut result2: u64 = 0;
    let mut i = 0;
    let mut j = 0;
    while i < matrix.len() {
        while j < matrix[i].len() {
            if matrix[i][j].is_digit(10) {
                let num = num_len((i, j), &matrix);
                let len = usize::try_from(num.checked_ilog10().unwrap_or(0) + 1).expect("Should be able to be a usize");
                if determine_part_number((i, j), len, &matrix) {
                    result1 += num;
                }
                j += len;
            } else if matrix[i][j] == '*' {
                result2 += u64::from(determine_gear_ratio((i, j), 1, &matrix));
                debug!("total ratio {result2} on {i}");
                j += 1;
            } else {
                j += 1;
            }
        }
        j = 0;

        debug!("{i}/{}", matrix.len());
        i += 1;
    }
    info!("Part number total = {result1}");
    info!("Gear ratio total = {result2}");
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("usage: {} <FILENAME>", args[0]);
        return; //TODO figure out returning errors without creating my own
    }

    let input = fs::read_to_string(&args[1]).expect("Should be able to read input");
    day3(input);
}
