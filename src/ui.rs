use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render_ui(app: &App, f: &mut Frame) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let title: Paragraph = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("✧ ", Style::default().fg(Color::Rgb(180, 120, 211))),
        Span::styled("Nebula Generator", Style::default().fg(Color::Rgb(180, 120, 211)).bold()),
        Span::styled(" ✧", Style::default().fg(Color::Rgb(180, 120, 211))),
    ]))).alignment(Alignment::Center);

    f.render_widget(title, main_chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Min(0),
        ]).split(main_chunks[1]);

    render_password_section(app, f, content_chunks[0]);
    render_settings_section(app, f, content_chunks[1]);
    render_controls_section(f, content_chunks[2]);
}

fn render_password_section(app: &App, f: &mut Frame, area: ratatui::layout::Rect) {
    let pass_block: Block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(79, 120, 204)))
        .title(Line::from(" Password ").centered().bold());

    let pass_text: Text = Text::from(vec![
        Line::from(app.password.clone()).centered().style(Style::default().bold()),
        Line::from(format!("Strength: {}", app.strength.description()))
            .centered()
            .style(Style::default().fg(app.strength.color())),
    ]);

    f.render_widget(Paragraph::new(pass_text).block(pass_block).alignment(Alignment::Center), area);
}

fn render_settings_section(app: &App, f: &mut Frame, area: ratatui::layout::Rect) {
    let settings_block: Block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(95, 179, 138)))
        .title(Line::from(" Settings ").centered().bold());

    let settings_text: Vec<Line> = vec![
        Line::from(vec![
            Span::raw("Length: "),
            Span::styled(app.length.to_string(), Style::default().bold().fg(Color::Rgb(105, 180, 210))),
        ]),

        Line::from(vec![
            Span::raw("Uppercase: "),
            Span::styled(
                if app.use_uppercase { "On" } else { "Off" },
                Style::default().bold().fg(
                    if app.use_uppercase { Color::Rgb(95, 179, 138) } else { Color::Rgb(217, 108, 108) }
                )
            ),
        ]),

        Line::from(vec![
            Span::raw("Numbers: "),
            Span::styled(
                if app.use_numbers { "On" } else { "Off" },
                Style::default().bold().fg(
                    if app.use_numbers { Color::Rgb(95, 179, 138) } else { Color::Rgb(217, 108, 108) }
                )
            ),
        ]),

        Line::from(vec![
            Span::raw("Symbols: "),
            Span::styled(
                if app.use_symbols { "On" } else { "Off" },
                Style::default().bold().fg(
                    if app.use_symbols { Color::Rgb(95, 179, 138) } else { Color::Rgb(217, 108, 108) }
                )
            ),
        ]),
    ];

    f.render_widget(Paragraph::new(settings_text).block(settings_block).alignment(Alignment::Center), area);
}

fn render_controls_section(f: &mut Frame, area: ratatui::layout::Rect) {
    let controls_block: Block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(209, 154, 102)))
        .title(Line::from(" Controls ").centered().bold());

    let key_style: Style = Style::default().bold().fg(Color::Rgb(209, 154, 102));

    let controls_text: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("+", key_style),
            Span::raw(" / "),
            Span::styled("-", key_style),
            Span::raw(": Increase/Decrease length"),
        ]),

        Line::from(vec![
            Span::styled("u", key_style),
            Span::raw(": Toggle Uppercase"),
        ]),

        Line::from(vec![
            Span::styled("n", key_style),
            Span::raw(": Toggle Numbers"),
        ]),

        Line::from(vec![
            Span::styled("s", key_style),
            Span::raw(": Toggle Symbols"),
        ]),

        Line::from(vec![
            Span::styled("g", key_style),
            Span::raw(": Generate new password"),
        ]),

        Line::from(vec![
            Span::styled("q", key_style),
            Span::raw(", "),
            Span::styled("Esc", key_style),
            Span::raw(", "),
            Span::styled("Ctrl-C", key_style),
            Span::raw(": Quit"),
        ]),
    ];

    f.render_widget(Paragraph::new(controls_text).block(controls_block).alignment(Alignment::Left), area);
}