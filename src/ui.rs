use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
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
        let block = Block::new();
        let mut file_buffer = self.buffers.get(self.current_file_path.clone());
        let file_buffer_content = file_buffer.file;
        let lines: Vec<String> = file_buffer_content
            .iter()
            .map(|line| line.iter().map(|c| c.to_string()).collect())
            .collect();
        let numbers: Vec<String> = (1..=lines.len()).map(|n| format!("{}.", n)).collect();
        let numbers_text = numbers.join("\n");
        let text = lines.join("\n");
        let scroll = (file_buffer.scroll_y, 0);

        file_buffer.scroll_y = file_buffer
            .current_line
            .saturating_sub(area.height - FOOTER_SIZE - 1);

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
            .block(block)
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
            let footer_text = Span::raw(format!("Mode: {}", self.editor_mode));
            buf.set_span(
                area.x + 1,
                footer_y,
                &footer_text,
                footer_text.width() as u16,
            );
        }
    }
}
