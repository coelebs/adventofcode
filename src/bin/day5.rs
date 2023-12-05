use aoclib::aocinit;
use log::{debug, info};
use std::collections::VecDeque;

fn map_step(seed: i64, line: &Vec<i64>) -> i64 {
    if seed >= line[1] && seed < (line[1] + line[2]) {
        return (seed - line[1]) + line[0];
    } else {
        return seed;
    }
}

fn process_section(section: &str, seedmap: &mut Vec<[i64; 8]>, step: usize) {
    debug!("Mapping {}", section.lines().next().unwrap());
    for line in section.lines().skip(1) {
        let map: Vec<i64> = line
            .split(" ")
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<i64>().unwrap_or(-1))
            .collect();

        for i in 0..seedmap.len() {
            let next = map_step(seedmap[i][step - 1], &map);
            if next != seedmap[i][step - 1] {
                seedmap[i][step] = next;
                debug!("{} to {next}", seedmap[i][step - 1]);
            }
        }
    }
}

fn part1(seeds: &Vec<i64>, mut sections: VecDeque<&str>) {
    let mut seedmap = vec![[-1; 8]; seeds.len()];

    for (idx, seed) in seeds.iter().enumerate() {
        seedmap[idx][0] = *seed;
    }

    let mut step = 1;
    while sections.len() > 0 {
        process_section(sections.pop_front().unwrap(), &mut seedmap, step);
        for sm in &mut seedmap {
            if sm[step] == -1 {
                sm[step] = sm[step - 1];
            }
        }
        step += 1;
    }

    let mut result1 = i64::MAX;
    seedmap.iter().for_each(|x| {
        if x[7] < result1 {
            result1 = x[7]
        }
    });
    info!("Lowest location seed: {result1}");
}

fn part2(_seeds: &Vec<i64>, mut sections: VecDeque<&str>) {
    //create ranges
    let mut ranges: Vec<Vec<Vec<i64>>> = vec![];
    ranges.resize(sections.len(), vec![vec![0]]);

    while sections.len() > 0 {
        let section = sections.pop_back().unwrap();

        section.lines().skip(1).for_each(|x| {
            ranges[sections.len()].push(x.split(" ").map(|x| x.parse::<i64>().unwrap()).collect())
        });
    }

    debug!("{:?}", ranges);
}

fn day5(input: String) {
    let mut sections = input.split("\n\n").collect::<VecDeque<&str>>();

    let seeds: Vec<i64> = sections
        .pop_front()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.parse::<i64>().unwrap_or(-1))
        .collect();

    part1(&seeds, sections.clone());
    part2(&seeds, sections);
}

fn main() {
    aocinit(day5);
}
