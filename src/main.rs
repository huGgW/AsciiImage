use clap::{Parser, SubCommand};

mod to_ascii;
mod save_ascii;
mod print_ascii;

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(SubCommand)]
enum Commands {
    #[clap(subcommand)]
    save(Save),
    #[clap(subcommand)]
    print(Print),
}

#[derive(SubCommand)]
struct Save {
    #[clap(short, long, default_value_t = String::from("./images/image.png"))]
    imgdir: String,
    #[clap(short, long, default_value_t = String::from("./result/result.txt"))]
    savedir: String,
    #[clap(short, long, default_value_t = 16u32)]
    size: u32,
    #[clap(short, long, action)]
    complex: bool,
    #[clap(short, long, action)]
    invert: bool,
}

#[derive(SubCommand)]
struct Print {
    #[clap(short, long, default_value_t = String::from("./images/image.png"))]
    imgdir: String,
    #[clap(short, long, default_value_t = 16u32)]
    size: u32,
    #[clap(short, long, action)]
    complex: bool,
    #[clap(short, long, action)]
    invert: bool,
}

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
