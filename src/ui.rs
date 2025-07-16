use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

use crate::app::Editor;

pub const LINE_NUMBERS_WIDTH: u16 = 5;
pub const FOOTER_SIZE: u16 = 2;

impl Widget for &Editor {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content_height = area.height.saturating_sub(FOOTER_SIZE);
        let mut file_buffer = self.buffers.get(self.current_file_path.clone());
        let file_buffer_content = file_buffer.file;
        let lines: Vec<String> = file_buffer_content
            .iter()
            .map(|line| line.iter().map(|c| c.to_string()).collect())
            .collect();
        let numbers: Vec<String> = (1..=lines.len()).map(|n| format!("{}.", n)).collect();
        let numbers_text = numbers.join("\n");
        let text = lines.join("\n");

        file_buffer.scroll_y = file_buffer
            .current_line
            .saturating_sub(area.height - FOOTER_SIZE - 1);

        let scroll = (file_buffer.scroll_y, 0);

        let line_numbers = Paragraph::new(numbers_text)
            .block(Block::new())
            .scroll(scroll)
            .fg(Color::from_u32(0x00969696))
            .bg(Color::Black);
        let line_numbers_area = Rect {
            x: area.x,
            y: area.y,
            width: LINE_NUMBERS_WIDTH,
            height: area.height - FOOTER_SIZE,
        };

        line_numbers.render(line_numbers_area, buf);

        let paragraph = Paragraph::new(text)
            .block(Block::new())
            .scroll(scroll)
            .fg(Color::Cyan)
            .bg(Color::Black);
        let paragraph_area = Rect {
            x: area.x + LINE_NUMBERS_WIDTH,
            y: area.y,
            width: area.width,
            height: area.height - FOOTER_SIZE,
        };

        paragraph.render(paragraph_area, buf);

        if content_height > FOOTER_SIZE {
            let footer_y = area.y + area.height - FOOTER_SIZE; // last line inside border
            let footer_text = Span::raw(format!(
                "Mode: {}    Current line : {}/{}",
                self.editor_mode,
                file_buffer.current_line + 1,
                file_buffer.lines_number,
            ));
            buf.set_span(
                area.x + 1,
                footer_y,
                &footer_text,
                footer_text.width() as u16,
            );
        }

        if self.command_popup.running {
            let block = Block::bordered()
                .title("Command pane")
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().bg(Color::Black).fg(Color::White));

            let command_paragraph =
                Paragraph::new("> ".to_string() + &self.command_popup.input_field.clone())
                    .block(block)
                    .fg(Color::Cyan)
                    .bg(Color::Black);

            command_paragraph.render(centered_rect(60, 20, area), buf);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical_chunks = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    let horizontal_chunks = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical_chunks[1]);

    horizontal_chunks[1]
}
