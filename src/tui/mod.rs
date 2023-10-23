use std::collections::{VecDeque, vec_deque};

use ratatui::{
    prelude::*,
    widgets::{Paragraph, Block, Borders},
};

use crate::{
    lc3::State,
    util::bits,
};

pub fn render_tui(lc3_state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Max(3),
                    //Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(75),
                ])
                .split(f.size());
            let top_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(30),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ])
                .split(outer_layout[1]);
            let bottom_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(outer_layout[2]);
            f.render_widget(
                Paragraph::new(format!("Current file: {}", lc3_state.filename))
                    .block(
                        Block::default()
                            .borders(Borders::all())
                    ),
                outer_layout[0],
            );
            //let text: Text = vec![Line::from("asdf")].into();
            let instruction_state: Text = vec![
                Line::from(format!("PC: x{:0>4X}", lc3_state.pc)),
                Line::from(format!("IR: x{:0<4X}", lc3_state.ir)),
                // TODO!
                // Line::from(format!("Decoded current instruction: x{:0<4X}", lc3_state.ir)),
            ].into();

            f.render_widget(
                Paragraph::new(instruction_state)
                    .block(
                        Block::default()
                            .borders(Borders::all())
                    ),
                top_layout[0],
            );

            let mut register_state: VecDeque<Text> = VecDeque::new();

            for i in 0..4 {
                let new_register: Text = vec![
                    Line::from(format!("R{}: x{:0>4X}", 2 * i, lc3_state.reg[2 * i])),
                    Line::from(format!("R{}: x{:0>4X}", 2 * i + 1, lc3_state.reg[2 * i + 1])),
                ].into();
                register_state.push_back(new_register);
            }

            for i in 2..6 {
                f.render_widget(
                    Paragraph::new(register_state.pop_front().unwrap())
                        .block(
                            Block::default()
                                .borders(
                                    Borders::TOP
                                        .union(Borders::BOTTOM)
                                        .union(
                                            if i == 2 { Borders::LEFT } else { Borders::NONE }
                                        )
                                        .union(
                                            if i == 5 { Borders::RIGHT } else { Borders::NONE }
                                        )
                                )
                        ),
                    top_layout[i],
                );
            }

        })?;

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        // crossterm::event::KeyCode::Char('j') => counter += 1,
                        // crossterm::event::KeyCode::Char('k') => counter -= 1,
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
