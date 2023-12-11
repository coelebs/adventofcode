use aoclib::aocinit;
use log::{debug, info};
use itertools::Itertools;
use std::collections::HashSet;

fn empty_space(matrix: &Vec<Vec<char>>) -> (HashSet<usize>, HashSet<usize>) {
    let mut empty_lines = HashSet::from_iter(0..matrix.len());
    let mut empty_columns = HashSet::from_iter(0..matrix[0].len());

    for (i, line) in matrix.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            if *x == '#' {
                empty_lines.remove(&i);
                empty_columns.remove(&j);
            }
        }
    }

    return (empty_lines, empty_columns);
}

fn count_empty_space(empty: &HashSet<usize>, i: usize) -> usize {
    let space = empty.iter().filter(|x| i > **x).count();

    if space > 0 {
        return (space * 1_000_000) - space;
    } else {
        return 0;
    }
}

fn find_galaxies(matrix: &Vec<Vec<char>>, empty_lines: HashSet<usize>, empty_columns: HashSet<usize>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for (i, line) in matrix.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            if *x == '#' {
                let new_i = i + count_empty_space(&empty_lines, i);
                let new_j = j + count_empty_space(&empty_columns, j);
                galaxies.push((new_i, new_j));
            }
        }
    }

    return galaxies;
}

fn count_difference(galaxies: Vec<&(usize, usize)>) -> usize {
    let mut result = 0;
    if galaxies.len() != 2 {
        panic!("rewrite");
    }

    if galaxies[0].0 > galaxies[1].0 {
        result += galaxies[0].0 - galaxies[1].0;
    } else {
        result += galaxies[1].0 - galaxies[0].0;
    }

    if galaxies[0].1 > galaxies[1].1 {
        result += galaxies[0].1 - galaxies[1].1;
    } else {
        result += galaxies[1].1 - galaxies[0].1;
    }

    return result;
}

fn day11(input: String) {
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut empty_lines = HashSet::new();
    let mut empty_columns = HashSet::new();
    let galaxies = find_galaxies(&matrix, empty_lines, empty_columns);
    debug!("{:?}", galaxies);
    debug!("{:?}", galaxies.iter().combinations(2).count());
    let result1 = galaxies.iter().combinations(2).fold(0, |acc, x| acc + count_difference(x));
    info!("Distance between all galaxies: {result1}");
    (empty_lines, empty_columns) = empty_space(&matrix);
    let galaxies = find_galaxies(&matrix, empty_lines, empty_columns);
    let result2 = galaxies.iter().combinations(2).fold(0, |acc, x| acc + count_difference(x));
    info!("Distance between expanded galaxies: {result2}");
}

fn main() {
    aocinit(day11);
}
