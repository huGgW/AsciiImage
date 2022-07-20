use std::fs::File;
use std::io::prelude::*;

mod to_ascii;
mod save_ascii;
mod print_ascii;

fn main() {
    let img_path = "images/ubuntu.png";
    let max_pixel = 16;
    let is_simple = true;
    let invert = false;

    let ascii_mat = to_ascii::transform(img_path, max_pixel, is_simple, invert);   

    let file_path = "./result/result.txt";

    save_ascii::save(file_path, &ascii_mat);
    print_ascii::print(&ascii_mat);
}
