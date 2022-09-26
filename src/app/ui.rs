use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph, List, ListItem, Wrap};
use tui::Frame;
use tui::text::Text;
use tui_logger::TuiLoggerWidget;

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use log::debug;

use crate::app::App;
use crate::app::state::AppState;

pub fn draw<B>(rect: &mut Frame<B>, _app: &mut App)
    where
        B: Backend,
{
    let size = rect.size();
    if !check_size(&size) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), ].as_ref())
            .split(size);

        let paragraph = Paragraph::new("Application need at least width of 52 and height of 28")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );

        rect.render_widget(paragraph, chunks[0]);
        return;
    }

    if *_app.state.display_help().unwrap() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), ].as_ref())
            .split(size);



        rect.render_widget(draw_help(), chunks[0]);
        return;
    }

    let display_log = *_app.state.display_log().unwrap();

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
                if display_log { Constraint::Length(10) } else { Constraint::Length(0) },
            ].as_ref())
        .split(size);

    // Title
    let str = _app.state.cursor().unwrap().to_str().unwrap();
    let title = draw_title(str);
    rect.render_widget(title, chunks[0]);

    if let AppState::Initialized { current_list, .. } = &mut _app.state {
        let mut list_items:Vec<ListItem> = Vec::new();

        for item in &current_list.items {
            list_items.push(
                ListItem::new(item.name.to_str().unwrap())
                    .style(
                        Style::default().fg(if item.is_dir {Color::Green} else {Color::White} )
                    )
            );
        }

        if current_list.state.selected().unwrap() >= current_list.items.len() {
            current_list.state.select(Some(0));
        }

        let list = draw_list(list_items);
        rect.render_stateful_widget(list, chunks[1], &mut current_list.state);
    }

    let selected_item = _app.state.current_list().unwrap()
        .items.get(_app.state.current_list().unwrap()
        .index()).unwrap()
        .clone();
    let datetime:DateTime<Utc> = selected_item.metadata.modified().unwrap().into();

    let datetime_str = format!("Modified: {}", datetime.format("%d/%m/%Y %T").to_string());
    let right_align_str = "[?] Help, Hello World";
    let whitespace_num = size.width as usize - datetime_str.len() - right_align_str.len();

    debug!("{}, {}, {}, {}", datetime_str.len(), right_align_str.len(), size.width, whitespace_num);

    let detail_str = String::from(format!("{}{:num$}{}", datetime_str, " ", right_align_str, num = whitespace_num));
    let detail = Paragraph::new(detail_str)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left);

    rect.render_widget(detail, chunks[2]);

    if display_log {
        let logs = draw_logs();
        rect.render_widget(logs, chunks[3]);
    }
}

fn draw_title(title: &str) -> Paragraph {
    Paragraph::new(Text::from(title))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
        )
}

fn draw_help<'a>() -> Paragraph<'a> {
    Paragraph::new("[q] Quit\n")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}

fn draw_list(list_items:Vec<ListItem>) -> List {
    List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        )
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
}

fn check_size(rect: &Rect) -> bool {
    return rect.width >= 52 && rect.height >= 28;
}