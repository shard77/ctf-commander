use std::error::Error;
use std::io;

use crate::api::platforms::hackthebox::{MachineProfile, ProfileInfo};
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::*;
use ratatui::Terminal;

pub fn print_machine_profile(machine_profile: MachineProfile) -> Result<(), Box<dyn Error>> {
    let machine_info = match machine_profile.info.get("info") {
        Some(info) => info,
        None => return Err("Machine info not found".into()),
    };

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_tab = 0;
    let tab_titles = vec![
        "Overview",
        "Stats",
        "Difficulty",
        "Creator",
        "Blood",
        "Academy",
    ];

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let tabs = Tabs::new(
                tab_titles
                    .clone()
                    .into_iter()
                    .map(Line::from)
                    .collect::<Vec<_>>(),
            )
            .select(current_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(symbols::DOT);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(size);

            f.render_widget(tabs, chunks[0]);

            match current_tab {
                0 => render_overview_tab(f, chunks[1], machine_info),
                1 => render_stats_tab(f, chunks[1], machine_info),
                2 => render_difficulty_tab(f, chunks[1], machine_info),
                3 => render_creator_tab(f, chunks[1], machine_info),
                4 => render_blood_tab(f, chunks[1], machine_info),
                5 => render_academy_tab(f, chunks[1], machine_info),
                _ => {}
            }

            let help_text = Line::from(vec![
                Span::styled("←/→", Style::default().fg(Color::Yellow)),
                Span::raw(" Switch tabs  "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" Quit"),
            ]);

            let help_paragraph = Paragraph::new(help_text)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);

            let help_area = Rect::new(
                chunks[1].x,
                chunks[1].y + chunks[1].height - 1,
                chunks[1].width,
                1,
            );

            f.render_widget(help_paragraph, help_area);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Left => {
                    if current_tab > 0 {
                        current_tab -= 1;
                    } else {
                        current_tab = tab_titles.len() - 1;
                    }
                }
                KeyCode::Right => {
                    if current_tab < tab_titles.len() - 1 {
                        current_tab += 1;
                    } else {
                        current_tab = 0;
                    }
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;

    Ok(())
}

fn render_overview_tab(f: &mut Frame, area: Rect, machine_info: &ProfileInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .margin(1)
        .split(area);

    let title = format!(" {} ", machine_info.name);
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan))
        .title(title)
        .title_alignment(Alignment::Center);

    f.render_widget(title_block, chunks[0]);

    let basic_info_rows = vec![
        Row::new(vec![
            Cell::from("ID").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.id.to_string()),
        ]),
        Row::new(vec![
            Cell::from("OS").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.os.as_str()),
        ]),
        Row::new(vec![
            Cell::from("IP").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.ip.as_deref().unwrap_or("N/A")),
        ]),
        Row::new(vec![
            Cell::from("Points").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.points.to_string()),
        ]),
        Row::new(vec![
            Cell::from("Release").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.release.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Retired").style(Style::default().fg(Color::Yellow)),
            Cell::from(if machine_info.retired { "Yes" } else { "No" }),
        ]),
        Row::new(vec![
            Cell::from("Free").style(Style::default().fg(Color::Yellow)),
            Cell::from(if machine_info.free { "Yes" } else { "No" }),
        ]),
    ];

    let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
    let basic_info_table = Table::new(basic_info_rows, widths)
        .block(Block::default().borders(Borders::ALL).title("Basic Info"))
        .column_spacing(1);

    f.render_widget(basic_info_table, chunks[1]);

    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let play_status = format!(
        "Active: {}\nSpawned: {}\nPlayers: {}",
        machine_info.play_info.is_active.unwrap_or(false),
        machine_info.play_info.is_spawned.unwrap_or(false),
        machine_info.play_info.active_player_count.unwrap_or(0)
    );

    let play_status_widget = Paragraph::new(play_status)
        .block(Block::default().borders(Borders::ALL).title("Play Status"))
        .style(Style::default().fg(Color::White));

    f.render_widget(play_status_widget, status_chunks[0]);

    let synopsis = match &machine_info.synopsis {
        Some(text) if !text.is_empty() => text.as_str(),
        _ => "No synopsis available",
    };

    let synopsis_widget = Paragraph::new(synopsis)
        .block(Block::default().borders(Borders::ALL).title("Synopsis"))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(synopsis_widget, status_chunks[1]);
}

fn render_stats_tab(f: &mut Frame, area: Rect, machine_info: &ProfileInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .margin(1)
        .split(area);

    let stats_rows = vec![
        Row::new(vec![
            Cell::from("User Owns").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.user_owns_count.to_string()),
        ]),
        Row::new(vec![
            Cell::from("Root Owns").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.root_owns_count.to_string()),
        ]),
        Row::new(vec![
            Cell::from("Reviews").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.reviews_count.to_string()),
        ]),
        Row::new(vec![
            Cell::from("Stars").style(Style::default().fg(Color::Yellow)),
            Cell::from(format!("{:.1} ★", machine_info.stars)),
        ]),
        Row::new(vec![
            Cell::from("Difficulty").style(Style::default().fg(Color::Yellow)),
            Cell::from(machine_info.difficulty_text.as_str()),
        ]),
    ];

    let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
    let stats_table = Table::new(stats_rows, widths)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Machine Statistics"),
        )
        .column_spacing(1);

    f.render_widget(stats_table, chunks[0]);

    let user_owns = machine_info.auth_user_in_user_owns.unwrap_or(false);
    let root_owns = machine_info.auth_user_in_root_owns.unwrap_or(false);
    let reviewed = machine_info.auth_user_has_reviewed;

    let user_progress = vec![
        ListItem::new(Line::from(vec![
            Span::styled("User Flag: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                if user_owns {
                    "✓ Owned"
                } else {
                    "✗ Not owned"
                },
                if user_owns {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Root Flag: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                if root_owns {
                    "✓ Owned"
                } else {
                    "✗ Not owned"
                },
                if root_owns {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Reviewed: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                if reviewed { "✓ Yes" } else { "✗ No" },
                if reviewed {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            ),
        ])),
    ];

    let user_progress_list = List::new(user_progress)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Your Progress"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">");

    f.render_widget(user_progress_list, chunks[1]);

    let mut extra_info = vec![];

    if let Some(time) = &machine_info.auth_user_first_user_time {
        extra_info.push(Line::from(vec![
            Span::styled("First user flag: ", Style::default().fg(Color::Yellow)),
            Span::raw(time),
        ]));
    }

    if let Some(time) = &machine_info.auth_user_first_root_time {
        extra_info.push(Line::from(vec![
            Span::styled("First root flag: ", Style::default().fg(Color::Yellow)),
            Span::raw(time),
        ]));
    }

    if extra_info.is_empty() {
        extra_info.push(Line::from("No completion times available"));
    }

    let extra_info_paragraph = Paragraph::new(extra_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Completion Times"),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(extra_info_paragraph, chunks[2]);
}

fn render_difficulty_tab(f: &mut Frame, area: Rect, machine_info: &ProfileInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .margin(1)
        .split(area);

    let difficulty_info = Paragraph::new(format!(
        "Difficulty Rating: {} ({})",
        machine_info.difficulty_text, machine_info.stars
    ))
    .block(Block::default().borders(Borders::ALL).title("Difficulty"))
    .style(Style::default().fg(Color::White));

    f.render_widget(difficulty_info, chunks[0]);

    let feedback = &machine_info.feedback_for_chart;

    let max_value = [
        feedback.counter_cake,
        feedback.counter_very_easy,
        feedback.counter_easy,
        feedback.counter_too_easy,
        feedback.counter_medium,
        feedback.counter_bit_hard,
        feedback.counter_hard,
        feedback.counter_too_hard,
        feedback.counter_ex_hard,
        feedback.counter_brain_fuck,
    ]
    .iter()
    .max()
    .copied()
    .unwrap_or(1);

    let data = [
        ("Cake", feedback.counter_cake as u64),
        ("Very Easy", feedback.counter_very_easy as u64),
        ("Easy", feedback.counter_easy as u64),
        ("Too Easy", feedback.counter_too_easy as u64),
        ("Medium", feedback.counter_medium as u64),
        ("Bit Hard", feedback.counter_bit_hard as u64),
        ("Hard", feedback.counter_hard as u64),
        ("Too Hard", feedback.counter_too_hard as u64),
        ("Ex Hard", feedback.counter_ex_hard as u64),
        ("Brain F", feedback.counter_brain_fuck as u64),
    ];

    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Difficulty Feedback"),
        )
        .data(&data[..])
        .bar_width(5)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Blue))
        .value_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));

    f.render_widget(barchart, chunks[1]);
}

fn render_creator_tab(f: &mut Frame, area: Rect, machine_info: &ProfileInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .margin(1)
        .split(area);

    let maker1_info = vec![
        ListItem::new(Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.maker.name),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("ID: ", Style::default().fg(Color::Yellow)),
            Span::raw(machine_info.maker.id.to_string()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Respected: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                if machine_info.maker.is_respected {
                    "Yes"
                } else {
                    "No"
                },
                if machine_info.maker.is_respected {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            ),
        ])),
    ];

    let maker1_list = List::new(maker1_info)
        .block(Block::default().borders(Borders::ALL).title("Creator"))
        .style(Style::default().fg(Color::White));

    f.render_widget(maker1_list, chunks[0]);

    if let Some(maker2) = &machine_info.maker2 {
        let maker2_info = vec![
            ListItem::new(Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                Span::raw(&maker2.name),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Yellow)),
                Span::raw(maker2.id.to_string()),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("Respected: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if maker2.is_respected { "Yes" } else { "No" },
                    if maker2.is_respected {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Red)
                    },
                ),
            ])),
        ];

        let maker2_list = List::new(maker2_info)
            .block(Block::default().borders(Borders::ALL).title("Co-Creator"))
            .style(Style::default().fg(Color::White));

        f.render_widget(maker2_list, chunks[1]);
    } else {
        let no_maker2 = Paragraph::new("No co-creator for this machine")
            .block(Block::default().borders(Borders::ALL).title("Co-Creator"))
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(no_maker2, chunks[1]);
    }
}

fn render_blood_tab(f: &mut Frame, area: Rect, machine_info: &ProfileInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(area);

    let user_blood_info = vec![
        ListItem::new(Line::from(vec![
            Span::styled("User: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.user_blood.user.name),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("User ID: ", Style::default().fg(Color::Yellow)),
            Span::raw(machine_info.user_blood.user.id.to_string()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Time: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.user_blood.created_at),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Difference: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.user_blood.blood_difference),
        ])),
    ];

    let user_blood_list = List::new(user_blood_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("User First Blood"),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(user_blood_list, chunks[0]);

    let root_blood_info = vec![
        ListItem::new(Line::from(vec![
            Span::styled("User: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.root_blood.user.name),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("User ID: ", Style::default().fg(Color::Yellow)),
            Span::raw(machine_info.root_blood.user.id.to_string()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Time: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.root_blood.created_at),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Difference: ", Style::default().fg(Color::Yellow)),
            Span::raw(&machine_info.root_blood.blood_difference),
        ])),
    ];

    let root_blood_list = List::new(root_blood_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Root First Blood"),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(root_blood_list, chunks[1]);
}

fn render_academy_tab(f: &mut Frame, area: Rect, machine_info: &ProfileInfo) {
    let modules = &machine_info.academy_modules;

    if modules.is_empty() {
        let no_modules = Paragraph::new("No academy modules associated with this machine")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Academy Modules"),
            )
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);

        f.render_widget(
            no_modules,
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );
        return;
    }

    let mut module_items = Vec::new();

    for module in modules {
        module_items.push(ListItem::new(vec![
            Line::from(vec![Span::styled(
                &module.name,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled("Tier: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{} ({})", module.tier.name, module.tier.number)),
            ]),
            Line::from(vec![
                Span::styled("Difficulty: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!(
                    "{} (Level {})",
                    module.difficulty.title, module.difficulty.level
                )),
            ]),
            Line::from(vec![
                Span::styled("URL: ", Style::default().fg(Color::Yellow)),
                Span::raw(&module.url),
            ]),
            Line::from(""),
        ]));
    }

    let modules_list = List::new(module_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Academy Modules ({})", modules.len())),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(
        modules_list,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
    );
}
