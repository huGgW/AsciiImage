use clap::Parser;

mod to_ascii;
mod save_ascii;
mod print_ascii;
mod cli_parse;

fn main() {
    let cli = cli_parse::Cli::parse();

    if let Some(Commands) = &cli.command {
        match Commands {
            cli_parse::Commands::Save(save) => {
                let ascii_mat = to_ascii::transform(&(save.img_path), save.size, !(save.complex), save.invert);
                save_ascii::save(&(save.save_path), &ascii_mat)
            },
            cli_parse::Commands::Print(print) => {
                let ascii_mat = to_ascii::transform(&(print.img_path), print.size, !(print.complex), print.invert);
                print_ascii::print(&ascii_mat);
            },
        }
    }
}
