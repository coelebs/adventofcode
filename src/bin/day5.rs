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

fn process_section_range(section: &str, seedmap: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    debug!("Mapping {}", section.lines().next().unwrap());
    let mut next = vec![];
    let bools = vec![false; seedmap.len()];
    let mut prev: Vec<((i64, i64), bool)> =
        seedmap.iter().zip(bools).map(|x| (*x.0, x.1)).collect();

    for line in section.lines().skip(1) {
        let map: Vec<i64> = line
            .split(" ")
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<i64>().unwrap_or(-1))
            .collect();

        let mut i = 0;
        while i < prev.len() {
            if !prev[i].1 {
                let start = prev[i].0 .0;
                let end = prev[i].0 .0 + prev[i].0 .1;

                let secstart = map[1];
                let secend = map[1] + map[2];

                if (start > secstart && start < secend) || (end > secstart && end < secend) {
                    prev[i].1 = true;
                    debug!("Overlap between: {start}:{end} and {secstart}:{secend}");
                    if start >= secstart && end <= secend {
                        debug!("Full overlap even");
                        next.push((start - secstart + map[0], prev[i].0 .1));
                    } else if start > secstart && start < secend {
                        debug!("Partial overlap right");
                        let matchend = secend - start;
                        next.push(((start - secstart) + map[0], matchend));
                        prev.push(((start + matchend, end - secend), false));
                        debug!("Rest is {},{}", start + matchend, end - secend);
                        debug!("Processed is {},{}", (start - secstart) + map[0], matchend);
                    } else if end > secstart && end < secend {
                        debug!("Partial overlap left");
                        next.push((map[0], end - secstart));
                        prev.push(((start, secstart - start), false));
                        debug!("Rest is {},{}", start, secstart - start);
                        debug!("Processed is {},{}", map[0], end - secstart);
                    }
                }
            }
            i += 1;
        }
    }

    for p in prev.iter().filter(|x| !x.1) {
        debug!("{:?} stays the same", p.0);
        next.push(p.0);
    }

    return next;
}

fn part2(seedranges: &mut Vec<(i64, i64)>, mut sections: VecDeque<&str>) {
    let starttotalseeds: i64 = seedranges.iter().map(|x| x.1).sum();
    let next = seedranges;
    while sections.len() > 0 {
        *next = process_section_range(sections.pop_front().unwrap(), &next);
        let totalseeds: i64 = next.iter().map(|x| x.1).sum();
        debug!("{totalseeds}/{starttotalseeds}");
        debug!("next: {:#?}", next);
    }

    let totalseeds: i64 = next.iter().map(|x| x.1).sum();
    debug!("{totalseeds}/{starttotalseeds}");

    let result2 = next.iter().map(|x| x.0).min().unwrap();
    info!("Ranges result: {result2}");
}

fn day5(input: String) {
    let mut sections = input.split("\n\n").collect::<VecDeque<&str>>();

    let seed_section = sections.pop_front().unwrap().split_once(":").unwrap().1;
    let seeds: Vec<i64> = seed_section
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.parse::<i64>().unwrap_or(-1))
        .collect();

    let mut seedranges: Vec<(i64, i64)> = vec![];
    part1(&seeds, sections.clone());
    let seed_in: Vec<&str> = seed_section.split_ascii_whitespace().collect();
    let mut i = 0;
    while i < seed_in.len() {
        seedranges.push((seed_in[i].parse().unwrap(), seed_in[i + 1].parse().unwrap()));
        i += 2;
    }
    debug!("{:#?}", seedranges);

    part2(&mut seedranges, sections);
}

fn main() {
    aocinit(day5);
}
