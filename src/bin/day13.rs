use aoclib::aocinit;
use log::{debug, info};

fn swap_field(input: char) -> char {
    if input == '#' {
        return '.';
    } else {
        return '#';
    }
}

fn find_vertical_reflection(matrix: &mut Vec<Vec<char>>) -> i64 {
    debug!("Find ver----");
    let mut columns = 0;
    let mut differences;
    let mut smudged = false;
    'outer: for i in 1..matrix[0].len() {
        differences = vec![];
        debug!("Trying on line {i}");
        for (row, line) in matrix.iter().enumerate() {
            for j in 0..std::cmp::min(line.len() - i, i) {
                if line[i + j] != line[i - (j + 1)] {
                    if differences.len() > 1 || smudged {
                        continue 'outer;
                    } else {
                        debug!("Smudge at {row},{}", i + j);
                        differences.push((row, i+j));
                    }
               }
            }
        }
        debug!("Found :)");
        if differences.len() <= 1 {
            columns += i as i64;
            if let Some((row, column)) = differences.pop() {
                debug!("Fix {row}{column}");
                matrix[row][column] = swap_field(matrix[row][column]);
                smudged = true;
            }
        }
    }

    debug!("------------");
    return columns;
}

fn find_horizontal_reflections(matrix: &mut Vec<Vec<char>>) -> i64 {
    debug!("Find hor----");
    let mut rows = 0;
    let mut differences;
    let mut smudged = false;
    'outer: for i in 1..matrix.len() {
        differences = vec![];
        for j in 0..std::cmp::min(matrix.len() - i, i) {
            debug!("Comparing line {} with {}", i + j, i - (j + 1));
            for k in 0..matrix[i].len() {
                if matrix[i + j][k] != matrix[i - (j + 1)][k] {
                    if differences.len() > 1 || smudged {
                        continue 'outer;
                    } else {
                        debug!("Smudge at {},{k}", i + j);
                        differences.push((i+j,k));
                    }
                }
            }
        }
        if differences.len() <= 1 {
            debug!("Found :)");
            rows += i as i64;
            if let Some((row, column)) = differences.pop() {
                debug!("Fix {row}{column}");
                matrix[row][column] = swap_field(matrix[row][column]);
                smudged = true;
            }
        }
    }

    debug!("------------");
    return rows;
}

fn day13(input: String) {
    let mut matrices: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect();
    debug!("{}", matrices.len());
    let mut vertmatrices = matrices.clone();

    let verts: i64 = vertmatrices
        .iter_mut()
        .map(|x| find_vertical_reflection(x))
        .sum();
    let hors: i64 = matrices
        .iter_mut()
        .map(|x| find_horizontal_reflections(x))
        .sum();
    debug!("{hors} - {verts}");
    let result1 = hors * 100 + verts;
    info!("{result1}");
}

fn main() {
    aocinit(day13);
}
