pub fn part1() -> Vec<u32> {
    let input = std::fs::read_to_string("day1.txt").unwrap();

    let elfs: Vec<u32> = input.split("\n\n").map(
        |elf| elf.split("\n")
        .map(|cals| cals.parse::<u32>().unwrap_or(0))
        .fold(0, |total, cal| total + cal)
        ).collect();

    println!("solution day1/part1 is: {}", elfs.iter().fold(std::u32::MIN, |a, b| a.max(*b)));
    return elfs;
}

pub fn part2() {
    let mut elfs = part1();
    elfs.sort_by(|a, b| b.cmp(a)); // sort descending
    println!("solution day1/part2 is: {}", &elfs[0..3].iter().fold(std::u32::MIN, |total, elf| total + elf));
}
