mod day1;

fn main() {
    if let Some(arg1) = std::env::args().nth(1) {
        if arg1 == "day1" {
            day1::part1();
            day1::part2();
        }
    }
}
