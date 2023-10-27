mod loader;
mod util;
mod lc3;
mod tui;

use loader::Filetype;
use tui::render_tui;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tui(TuiArgs),
}

#[derive(Args)]
struct TuiArgs {
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tui(tui_args) => {
            let f = Filetype::PlaintextBinary(tui_args.file.as_str());
            let results = f.parse_to_word_array()?;

            let mut state = lc3::State {
                filename: tui_args.file.as_str(),
                pc: 0x3000u16 as i16,
                ir: 0x0000,
                mem: results,
                reg: [0x8888u16 as i16; 8],
                psr: 0b1_0000_111_00000_000u16 as i16,
            };

            render_tui(&mut state)?;
        }
    }
    Ok(())
}

