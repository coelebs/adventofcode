use aoclib::aocinit;
use std::collections::HashSet;
use log::{info, debug};

#[derive(PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Step {
    row: usize,
    column: usize,
    direction: Direction,
}

// .
fn walk_normal(step: Step, limits: (usize, usize)) -> Option<Step> {
    return match step.direction {
        Direction::Up => if step.row == 0 { None } else { Some(Step{row: step.row - 1, column: step.column, direction: step.direction})},
        Direction::Left => if step.column == 0 { None } else { Some(Step{row: step.row, column: step.column - 1, direction: step.direction})},
        Direction::Down => if step.row == limits.0 { None } else { Some(Step{row: step.row + 1, column: step.column, direction: step.direction})},
        Direction::Right => if step.column == limits.1 { None } else { Some(Step{row: step.row, column: step.column + 1, direction: step.direction})},
    };
}

// /
fn walk_left_mirror(step: Step, limits: (usize, usize)) -> Option<Step> {
    return match step.direction {
        Direction::Up => if step.column == limits.1 { None } else { Some(Step{row: step.row, column: step.column + 1, direction: Direction::Right})},
        Direction::Left => if step.row == limits.0 { None } else { Some(Step{row: step.row + 1, column: step.column, direction: Direction::Down})},
        Direction::Down => if step.column == 0 { None } else { Some(Step{row: step.row, column: step.column - 1, direction: Direction::Left})},
        Direction::Right => if step.row == 0 { None } else { Some(Step{row: step.row - 1, column: step.column, direction: Direction::Up})},
    };
}

// \
fn walk_right_mirror(step: Step, limits: (usize, usize)) -> Option<Step> {
    return match step.direction {
        Direction::Up => if step.column == 0 { None } else { Some(Step{row: step.row, column: step.column - 1, direction: Direction::Left})},
        Direction::Left => if step.row == 0 { None } else { Some(Step{row: step.row - 1, column: step.column, direction: Direction::Up})},
        Direction::Down => if step.column == limits.1 { None } else { Some(Step{row: step.row, column: step.column + 1, direction: Direction::Right})},
        Direction::Right => if step.row == limits.0 { None } else { Some(Step{row: step.row + 1, column: step.column, direction: Direction::Down})},
    };
}

// |
fn walk_pipe(step: Step, limits: (usize, usize)) -> (Option<Step>, Option<Step>) {
    return match step.direction {
        Direction::Up => if step.row == 0 { (None, None) } else { (Some(Step{row: step.row - 1, column: step.column, direction: step.direction}), None)},
        Direction::Down => if step.row == limits.0 { (None, None) } else { (Some(Step{row: step.row + 1, column: step.column, direction: step.direction}), None)},
        Direction::Left | Direction::Right =>
            if step.row == 0 {
                (Some(Step{row: step.row + 1, column: step.column, direction: Direction::Down}), None)
            } else if step.row == limits.0 {
                (Some(Step{row: step.row - 1, column: step.column, direction: Direction::Up}), None)
            } else {
                (Some(Step{row: step.row + 1, column: step.column, direction: Direction::Down}),
                 Some(Step{row: step.row - 1, column: step.column, direction: Direction::Up}))
            },
    };
}

// -
fn walk_bar(step: Step, limits: (usize, usize)) -> (Option<Step>, Option<Step>) {
    return match step.direction {
        Direction::Left => if step.column == 0 { (None, None) } else { (Some(Step{row: step.row, column: step.column - 1, direction: step.direction}), None)},
        Direction::Right => if step.column == limits.1 { (None, None) } else { (Some(Step{row: step.row, column: step.column + 1, direction: step.direction}), None)},
        Direction::Down | Direction::Up =>
            if step.column == 0 {
                (Some(Step{row: step.row, column: step.column + 1, direction: Direction::Right}), None)
            } else if step.column == limits.1 {
                (Some(Step{row: step.row, column: step.column - 1, direction: Direction::Left}), None)
            } else {
                (Some(Step{row: step.row, column: step.column + 1, direction: Direction::Right}),
                 Some(Step{row: step.row, column: step.column - 1, direction: Direction::Left}))
            },
    };
}

fn walk(step: Step, maze: &Vec<Vec<char>>, steps: &mut HashSet<Step>, limits: (usize, usize)) {
    debug!("{},{}", step.row, step.column);
    let current = maze[step.row][step.column];
    let mut next = vec![];
    match current {
        '.' => next.push(walk_normal(step, limits)),
        '/' => next.push(walk_left_mirror(step, limits)),
        '\\' => next.push(walk_right_mirror(step, limits)),
        '|' => { let both = walk_pipe(step, limits); next.push(both.0); next.push(both.1) },
        '-' => { let both = walk_bar(step, limits); next.push(both.0); next.push(both.1) },
        _ => panic!("Huh"),
    };

    for nextstep in next {
        if nextstep.is_some() {
            let len = steps.len();
            steps.insert(nextstep.clone().unwrap());
            if len == steps.len() {
                return;
            }
            walk(nextstep.unwrap(), maze, steps, limits);
        }
    }
}

fn day16(input: String) {
    let maze: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut steps: HashSet<Step> = HashSet::new();
    let limits = (maze.len() - 1, maze[0].len() - 1);

    let firststep = Step{row: 0, column: 0, direction: Direction::Right};
    steps.insert(firststep.clone());
    walk(firststep, &maze, &mut steps, limits);
    let result1 = steps.iter().map(|x| (x.row, x.column)).collect::<HashSet<(usize, usize)>>();
    for i in 0..limits.0+1 {
        for j in 0..limits.1+1 {
            if result1.contains(&(i,j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    info!("Number of energized fields: {}", result1.len());
}

fn main() {
    aocinit(day16);
}
