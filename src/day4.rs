fn range(range: &str) -> Vec<u32> {
    return range
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
}

fn full_overlap(ranges: &str) -> u32 {
    let elves: Vec<Vec<u32>> = ranges.split(",").map(|x| range(x)).collect();

    if (elves[0][0] >= elves[1][0] && elves[0][1] <= elves[1][1])
        || (elves[1][0] >= elves[0][0] && elves[1][1] <= elves[0][1])
    {
        return 1;
    } else {
        return 0;
    }
}

fn any_overlap(ranges: &str) -> u32 {
    let elves: Vec<Vec<u32>> = ranges.split(",").map(|x| range(x)).collect();

    if (elves[0][0] >= elves[1][0] && elves[0][0] <= elves[1][1])
        || (elves[0][1] >= elves[1][0] && elves[0][1] <= elves[1][1])
        || (elves[1][0] >= elves[0][0] && elves[1][0] <= elves[0][1])
        || (elves[1][1] >= elves[0][0] && elves[1][1] <= elves[0][1])
    {
        return 1;
    } else {
        return 0;
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("day4.txt")
        .expect("There should be a day4.txt file containing puzzle input");

    println!(
        "Solution day4/part1: {}",
        input
            .lines()
            .fold(std::u32::MIN, |total, x| total + full_overlap(x))
    );
}

pub fn part2() {
    let input = std::fs::read_to_string("day4.txt")
        .expect("There should be a day4.txt file containing puzzle input");

    println!(
        "Solution day4/part2: {}",
        input
            .lines()
            .fold(std::u32::MIN, |total, x| total + any_overlap(x))
    );
}
