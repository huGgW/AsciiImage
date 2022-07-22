use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Save transformed ascii file to text.
    Save(Save),
    /// Print transformed ascii text on console.
    Print(Print),
}

#[derive(Args)]
#[clap(author, version, about)]
pub struct Save {
    #[clap(short, long, default_value_t = String::from("./images/image.png"))]
    /// Path of the image file.
    pub img_path: String,
    #[clap(short, long, default_value_t = String::from("./result/result.txt"))]
    /// Path to save the transformed ascii text file.
    pub save_path: String,
    #[clap(long, default_value_t = 32u32)]
    /// Maximum height of the transformed ascii text file.
    pub size: u32,
    #[clap(long, action)]
    /// Use more variation of characters.
    pub complex: bool,
    #[clap(long, action)]
    /// Invert the photo.
    pub invert: bool,
}

#[derive(Args)]
#[clap(author, version, about)]
pub struct Print {
    #[clap(short, long, default_value_t = String::from("./images/image.png"))]
    /// Path of the image file.
    pub img_path: String,
    #[clap(long, default_value_t = 32u32)]
    /// Maximum height of the transformed ascii text file.
    pub size: u32,
    #[clap(long, action)]
    /// Use more variation of characters.
    pub complex: bool,
    #[clap(long, action)]
    /// Invert the photo.
    pub invert: bool,
}