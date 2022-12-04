mod day1;
mod day2;

fn main() {
    if let Some(arg1) = std::env::args().nth(1) {
        if arg1 == "day1" {
            day1::part1();
            day1::part2();
        } else if arg1 == "day2" {
            day2::part1();
            day2::part2();
        }
    }
}
