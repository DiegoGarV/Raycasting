use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn is_wall(i: usize, j: usize, maze: &Vec<Vec<char>>) -> bool {
    if i >= maze[0].len() || j >= maze.len() {
        return true;
    }
    maze[j][i] != ' '
}