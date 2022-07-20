use image::io::Reader as ImageReader;
use image::imageops::FilterType as Filter;

static SIMPLE_BRIGHTNESS_CHAR: [char; 10] = [' ','.',':','-','=','+','*','#','%','@'];
static COMPLEX_BRIGHTNESS_CHAR: [char; 69] = [' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

pub fn transform(path: &str, max_pixel: u32, is_simple: bool, invert: bool) -> Vec<Vec<char>> {
    let brightness_char;
    if is_simple {
        brightness_char = SIMPLE_BRIGHTNESS_CHAR.to_vec();
    } else {
        brightness_char = COMPLEX_BRIGHTNESS_CHAR.to_vec();
    }

    let img = ImageReader::open(path)
        .expect("No image file exists").decode().expect("Decoding error occured");
    let mut luma_img = image::imageops::resize(
        &img.into_luma8(), max_pixel, max_pixel, Filter::Lanczos3
    );
    if invert { image::imageops::invert(&mut luma_img); }

    let mut bright_mat: Vec<Vec<u8>> = vec![vec![0u8; (&luma_img).width() as usize]; (&luma_img).height() as usize];

    for (x, y, pixel) in luma_img.enumerate_pixels() {
        (&mut bright_mat)[y as usize][x as usize] = ((pixel[0] as f64) * ((brightness_char.len() - 1) as f64) / 255.0) as u8;
    }

    let mut ascii_mat: Vec<Vec<char>> = Vec::new();
    for row in bright_mat {
        ascii_mat.push(Vec::new());
        let ascii_row = ascii_mat.last_mut().unwrap();
        for idx in row {
            ascii_row.push(brightness_char[idx as usize]);
        }
    }

    ascii_mat
}