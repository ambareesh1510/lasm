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

    let mut state = lc3::State {
        pc: 0x3000u16 as i16,
        ir: 0x0000,
        mem: results,
        reg: [0x8888u16 as i16; 8],
        psr: 0b1_0000_111_00000_000u16 as i16,
    };

    state.print();
    state.execute_next_instruction()?;
    state.execute_next_instruction()?;
    state.print();
    Ok(())
}

