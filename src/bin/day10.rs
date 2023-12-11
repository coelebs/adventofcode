use aoclib::aocinit;
use std::collections::HashSet;
use log::{debug, info};

#[derive(PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

fn get_next_coordinate(tile: &char, direction: &Direction) -> Direction {
    return match tile {
        '|' => if *direction == Direction::Up { Direction::Up } else { Direction::Down },
        '-' => if *direction == Direction::Left { Direction::Left } else { Direction::Right },
        'L' => if *direction == Direction::Down { Direction::Right } else { Direction::Up },
        'J' => if *direction == Direction::Down { Direction::Left } else { Direction::Up },
        '7' => if *direction == Direction::Up { Direction::Left } else { Direction::Down },
        'F' => if *direction == Direction::Up { Direction::Right } else { Direction::Down },
        _ => panic!("Should not be in path")
    };
}

fn walk(matrix: &Vec<Vec<char>>, mut start: (usize, usize)) -> (Vec<char>, HashSet<(usize, usize)>) {
    let mut result = vec![];
    let mut enclosed_result: (HashSet<(usize, usize)>, HashSet<(usize, usize)>) = (HashSet::new(), HashSet::new());
    let turns = 0;
    result.push('S');
    start.0 += 1; //Loop always starts going down
    let mut direction = Direction::Down;

    while matrix[start.0][start.1] != 'S' {
        result.push(matrix[start.0][start.1]);

        direction = get_next_coordinate(&matrix[start.0][start.1], &direction);
        match direction {
            Direction::Up => start.0 -= 1,
            Direction::Down => start.0 += 1,
            Direction::Left => start.1 -= 1,
            Direction::Right => start.1 += 1,
        }
    }

    if turns > 0 {
        return (result, enclosed_result.0);
    } else {
        return (result, enclosed_result.1);
    }
}

fn find_start(matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Some((i,j));
            }
        }
    }

    return None;
}

fn day10(input: String) {
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let start = find_start(&matrix).unwrap();
    debug!("Start: {:?}", start);

    let result = walk(&matrix, start);
    info!("Furthest walk = {}", result.0.len() / 2);
    info!("Enclosed locations = {}", result.1.len());
}

fn main() {
    aocinit(day10);
}
