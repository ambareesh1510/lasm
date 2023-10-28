use std::collections::{vec_deque, VecDeque};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{lc3::State, util::bits};

pub fn render_tui(lc3_state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut memory_render_offset = 0usize;
    let mut memory_render_window_width = 0usize;

    let mut memory_traverse_mode = false;
    let mut memory_traverse_address = 0usize;

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Max(3),
                    Constraint::Percentage(80),
                    Constraint::Max(2),
                ])
                .split(f.size());

            let bottom_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(30),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(50),
                ])
                .split(outer_layout[1]);

            let instructions_and_registers_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ])
                .split(bottom_layout[0]);

            memory_render_window_width = (bottom_layout[1].height - 3) as usize;

            f.render_widget(
                Paragraph::new(format!("Current file: {}", lc3_state.filename))
                    .block(Block::default().borders(Borders::all())),
                outer_layout[0],
            );

            let instruction_state: Text = vec![
                Line::from(format!("PC: x{:0>4X}", lc3_state.pc)),
                Line::from(format!("IR: x{:0<4X}", lc3_state.ir)),
                Line::from(format!(
                    "CC: {}",
                    // bits(lc3_state.psr, 2, 0)
                    if bits(lc3_state.psr, 2, 2) == 1 {
                        "n"
                    } else if bits(lc3_state.psr, 1, 1) == 1 {
                        "z"
                    } else if bits(lc3_state.psr, 0, 0) == 1 {
                        "p"
                    } else {
                        "-"
                    },
                )),
                // TODO!
                // Line::from(format!("Decoded current instruction: x{:0<4X}", lc3_state.ir)),
            ]
            .into();

            f.render_widget(
                Paragraph::new(instruction_state).block(
                    Block::default()
                        .title(" instruction data ")
                        .borders(Borders::all()),
                ),
                instructions_and_registers_layout[0],
            );

            let mut register_state: Vec<Line> = vec![];

            for i in 0..8 {
                register_state.push(Line::from(format!("R{}: x{:0>4X}", i, lc3_state.reg[i])));
            }

            f.render_widget(
                Paragraph::new(register_state)
                    .block(Block::default().title(" registers ").borders(Borders::ALL)),
                instructions_and_registers_layout[1],
            );

            let keybinds: Vec<Line> = vec![
                Line::from("j/k: scroll memory viewer up/down"),
                Line::from("n: execute next instruction"),
                Line::from("q: quit"),
            ];

            f.render_widget(
                Paragraph::new(keybinds)
                    .block(Block::default().title(" keybinds ").borders(Borders::ALL)),
                instructions_and_registers_layout[2],
            );

            let mut memory_addresses: Vec<Line> = vec![];
            let mut memory_values: Vec<Line> = vec![];

            memory_addresses.push(Line::from("Address"));
            memory_values.push(Line::from("Value"));

            for i in memory_render_offset..(memory_render_window_width + memory_render_offset) {
                memory_addresses.push(Line::from(format!("x{:0>4X}", i)));
                memory_values.push(Line::from(format!("x{:0>4X}", lc3_state.mem[i])));
            }

            f.render_widget(
                Paragraph::new(memory_addresses).block(
                    Block::default()
                        .title(" memory viewer ")
                        .borders(Borders::LEFT.union(Borders::TOP).union(Borders::BOTTOM)),
                ),
                bottom_layout[1],
            );

            f.render_widget(
                Paragraph::new(memory_values).block(
                    Block::default()
                        .borders(Borders::RIGHT.union(Borders::TOP).union(Borders::BOTTOM)),
                ),
                bottom_layout[2],
            );

            f.render_widget(
                Paragraph::new("coming soon!")
                    .block(Block::default().title(" console ").borders(Borders::ALL)),
                bottom_layout[3],
            );

            f.render_widget(
                Paragraph::new(if !memory_traverse_mode {
                    String::from("")
                } else if memory_traverse_address == 0 {
                    String::from(":goto x")
                } else {
                    format!(":goto x{:X}", memory_traverse_address)
                })
                .block(Block::default().borders(Borders::NONE)),
                outer_layout[2],
            );
        })?;

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    if memory_traverse_mode {
                        match key.code {
                            crossterm::event::KeyCode::Char(e) => {
                                if let Some(digit) = e.to_digit(16) {
                                    memory_traverse_address = memory_traverse_address
                                        .saturating_mul(16)
                                        .saturating_add(digit as usize);
                                }
                            }
                            crossterm::event::KeyCode::Backspace => {
                                memory_traverse_address /= 16;
                            }
                            crossterm::event::KeyCode::Esc => {
                                memory_traverse_mode = false;
                                memory_traverse_address = 0;
                            }
                            crossterm::event::KeyCode::Enter => {
                                if memory_traverse_address + memory_render_window_width < 65536 {
                                    memory_render_offset = memory_traverse_address;
                                } else {
                                    memory_render_offset = 65536 - memory_render_window_width;
                                }
                            }
                            _ => {}
                        }
                    }
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => {
                            if memory_render_offset + memory_render_window_width < 65536 {
                                memory_render_offset += 1;
                            }
                        }
                        crossterm::event::KeyCode::Char('k') => {
                            memory_render_offset = memory_render_offset.saturating_sub(1);
                        }
                        crossterm::event::KeyCode::Char('n') => {
                            lc3_state.execute_next_instruction()?;
                            // terminal.clear()?;
                        }
                        crossterm::event::KeyCode::Char(':') => {
                            memory_traverse_mode = true;
                            memory_traverse_address = 0;
                        }
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
