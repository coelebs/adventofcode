use aoclib::aocinit;
use log::{debug, info};

fn find_vertical_reflection(matrix: &Vec<Vec<char>>) -> i64 {
    debug!("Find ver----");
    let mut columns = 0;
    'outer: for i in 1..matrix[0].len() {
        debug!("Trying on line {i}");
        for line in matrix {
            for j in 0..std::cmp::min(line.len() - i, i) {
                if line[i + j] != line[i - (j + 1)] {
                    continue 'outer;
                }
            }
        }
        debug!("Found :)");
        columns += i as i64;
    }

    debug!("------------");
    return columns;
}

fn find_horizontal_reflections(matrix: &Vec<Vec<char>>) -> i64 {
    debug!("Find hor----");
    let mut rows = 0;
    'outer: for i in 1..matrix.len() {
        for j in 0..std::cmp::min(matrix.len() - i, i) {
            debug!("Comparing line {} with {}", i + j, i - (j + 1));
            for k in 0..matrix[i].len() {
                if matrix[i + j][k] != matrix[i - (j + 1)][k] {
                    continue 'outer;
                }
            }
        }
        debug!("Found :)");
        rows += i as i64;
    }

    debug!("------------");
    return rows;
}

fn day13(input: String) {
    let matrices: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect();
    debug!("{}", matrices.len());

    let verts: i64 = matrices
        .iter()
        .map(|x| find_vertical_reflection(x))
        .sum();
    let hors: i64 = matrices
        .iter()
        .map(|x| find_horizontal_reflections(x))
        .sum();
    debug!("{hors} - {verts}");
    let result1 = hors * 100 + verts;
    info!("{result1}");
}

fn main() {
    aocinit(day13);
}
