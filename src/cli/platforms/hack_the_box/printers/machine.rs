use crate::api::platforms::hackthebox::{MachineList, MachineProfile};
use std::{
    collections::{BTreeMap, VecDeque},
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

// TODO, complete printers

use ratatui::{prelude::*, widgets::*};

pub fn print_machine_profile(machine_profile: MachineProfile) -> Result<(), Box<dyn Error>> {
    let machine_info = &machine_profile.info["info"];

    crossterm::terminal::enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(8),
        },
    )?;

    let rows = [
        Row::new(vec![
            Cell::from("Property").fg(Color::Cyan).bold(),
            Cell::from("Value").fg(Color::Cyan).bold(),
        ]),
        Row::new(vec![Cell::from("Name"), Cell::from("XXXX")]),
        Row::new(vec![Cell::from("IP Address"), Cell::from("test")]),
        Row::new(vec![Cell::from("OS"), Cell::from("tttt")]),
        Row::new(vec![Cell::from("Points"), Cell::from("testtest")]),
    ];
    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .footer(Row::new(vec!["Updated on Dec 28"]));

    terminal.draw(|f| {
        f.render_widget(Clear, f.size());
        f.render_widget(table, f.size());
    })?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

pub fn print_machine_list(machine_list: MachineList) {}
