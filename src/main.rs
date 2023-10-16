/*
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
*/
// use std::io;

mod loader;
mod util;
mod lc3;

use loader::Filetype;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = Filetype::PlaintextBinary("test.lbin");
    let results = f.parse_to_word_array()?;
    println!("{:b}", results[0]);

    let mut state = lc3::State {
        pc_star: 0x3000,
        ir: 0x0000,
        mem: results,
        reg: [0; 8],
        psr: 0b1000011100000111,
    };

    state.print();
    state.execute_next_instruction()?;
    Ok(())
}

