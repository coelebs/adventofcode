use aoclib::aocinit;
use log::{debug, info};

fn arrangements_opts(mut input: String) -> Vec<String> {
    let mut result = vec![];
    let index = input.find("?");
    if index.is_none() {
        result.push(input);
        return result;
    }

    let i = index.unwrap();

    input.replace_range(i..i + 1, ".");
    result.extend(arrangements_opts(input.clone()));
    input.replace_range(i..i + 1, "#");
    result.extend(arrangements_opts(input.clone()));

    return result;
}

fn arrangement_combination(input_combination: &Vec<u32>, combinations: &Vec<u32>) -> bool {
    if input_combination.len() != combinations.len() {
        return false;
    }

    let mut result = true;
    for i in 0..input_combination.len() {
        if input_combination[i] != combinations[i] {
            result = false;
        }
    }

    return result;
}

fn day12(input: String) {
    let arrangements: Vec<(String, Vec<u32>)> = input
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| {
            (
                x.0.to_string(),
                x.1.split(",").map(|x| x.parse::<u32>().unwrap()).collect(),
            )
        })
        .collect();
    debug!("Arrangements: {:#?}", arrangements);

    let result1: usize = arrangements
        .iter()
        .map(|y| {
            arrangements_opts(y.0.clone())
                .iter()
                .map(|x| {
                    x.split(".")
                        .filter(|x| x.len() > 0)
                        .map(|x| x.len() as u32)
                        .collect::<Vec<u32>>()
                })
                .filter(|x| arrangement_combination(x, &y.1))
                .count()
        })
        .sum();
    info!("Sum of arrangements: {result1}");
}

fn main() {
    aocinit(day12);
}
