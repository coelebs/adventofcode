mod day1;
mod day2;
mod day3;

fn main() {
    if let Some(arg1) = std::env::args().nth(1) {
        if arg1 == "day1" {
            day1::part1();
            day1::part2();
        } else if arg1 == "day2" {
            day2::part1();
            day2::part2();
        } else if arg1 == "day3" {
            day3::part1();
            day3::part2();
        }
    }
}
