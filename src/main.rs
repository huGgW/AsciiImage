use std::fs::File;
use std::io::prelude::*;

mod to_ascii;

fn main() {
    let path = "./images/arch.png";
    let max_pixel = 64;
    let is_simple = true;
    let invert = true;

    let ascii_mat = to_ascii::transform(path, max_pixel, is_simple, invert);   

    let mut file = File::create("./result/result.txt").unwrap();

    for row in ascii_mat {
        // for make it square, twice per char 
        for c in row {
            write!(file, "{}{}", c, c).unwrap();
        }
        writeln!(file).unwrap();
    }

}
