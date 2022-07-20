use std::fs::File;
use std::io::prelude::*;

pub fn save(path: &str, ascii_mat: &Vec<Vec<char>>) {
    let mut file = File::create(path).expect("Failed creating file");

    for row in ascii_mat {
        // for make it square, twice per char 
        for c in row {
            write!(file, "{}{}", c, c).expect("Failed writing in file");
        }
        writeln!(file).expect("Failed writing in file");
    }
}