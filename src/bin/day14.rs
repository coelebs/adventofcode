use aoclib::aocinit;
use log::{debug, info};

fn tilt_dish(dish: &mut Vec<Vec<char>>) {
    for column in dish {
        let mut prev_square = 0;
        let mut rocks = 0;
        for i in 0..column.len() {
            if column[i] == 'O' {
                rocks += 1;
            }

            if column[i] == '#' {
                for i in prev_square..i {
                    if rocks > 0 {
                        column[i] = 'O';
                        rocks -= 1;
                    } else {
                        column[i] = '.';
                    }
                }

                prev_square = i+1;
            }
        }

        for i in prev_square..column.len() {
            if rocks > 0 {
                column[i] = 'O';
                rocks -= 1;
            } else {
                column[i] = '.';
            }
        }
    }
}

fn count_load(dish: &Vec<Vec<char>>) -> u32 {
    let mut rock_load = 0;
    for column in dish {
        let mut relative_load = column.len() as u32;
        for el in column {
            if *el == 'O' {
                rock_load += relative_load;
                debug!("{rock_load}");
            }
            relative_load -= 1;
        }
        debug!("--");
    }

    return rock_load;
}

fn turn_table(dish: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = dish[0].len();
    let mut turned_dish: Vec<Vec<char>> = vec![vec![]; len];
    for line in dish {
        for (index, char) in line.iter().enumerate() {
            turned_dish[index].push(*char);
        }
    }

    return turned_dish;
}

fn day14(input: String) {
    let len = input.lines().nth(0).unwrap().len();
    let mut dish: Vec<Vec<char>> = vec![vec![]; len];
    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            dish[index].push(char);
        }
    }

    debug!("dish: {:#?}", dish);
    tilt_dish(&mut dish);
    debug!("dish: {:#?}", dish);
    let result1 = count_load(&dish);
    info!("North support beam load: {result1}");
}

fn main() {
    aocinit(day14);
}
