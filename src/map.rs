use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_map(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("No se pudo abrir el archivo de mapa");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Error al leer la línea").chars().collect())
        .collect()
}
