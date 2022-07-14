use std::fs::File;
use std::io::prelude::*;
use image::io::Reader as ImageReader;
use image::imageops::FilterType as Filter;

// static brightnessChar: [char; 10] = [' ','.',':','-','=','+','*','#','%','@'];
static brightnessChar: [char; 69] = [' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

fn main() {
    let imagePath = "./images/kirby_pixel.png";
    let pixel = 64;


    let img = ImageReader::open(imagePath)
        .expect("No file exists\n").decode().expect("Decoding error occured\n");
    let lumaImg = image::imageops::resize(
        &img.into_luma8(), pixel, pixel, Filter::Lanczos3
    );

    let mut brightMat: Vec<Vec<u8>> = vec![vec![0u8; (&lumaImg).width() as usize]; (&lumaImg).height() as usize];

    for (x, y, pixel) in lumaImg.enumerate_pixels() {
        (&mut brightMat)[y as usize][x as usize] = ((pixel[0] as f64) * ((brightnessChar.len() - 1) as f64) / 255.0) as u8;
    }

    let mut asciiMat: Vec<Vec<char>> = Vec::new();
    for row in brightMat {
        asciiMat.push(Vec::new());
        let mut asciiRow = asciiMat.last_mut().unwrap();
        for idx in row {
            asciiRow.push(brightnessChar[idx as usize]);
        }
    }

    let mut file = File::create("result.txt").unwrap();

    for row in asciiMat {
        // for make it square, twice per char 
        for c in row {
            write!(file, "{}{}", c, c).unwrap();
        }
        writeln!(file, "").unwrap();
    }

}
