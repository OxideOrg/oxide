use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

use crate::app::Editor;

impl Widget for &Editor {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content_height = area.height.saturating_sub(2);
        let block = Block::new();
        let file_buffer = self.buffers.get(self.current_file_path.clone()).file;
        let lines: Vec<String> = file_buffer
            .iter()
            .map(|line| line.iter().map(|c| c.to_string()).collect())
            .collect();
        let text = lines.join("\n");

        let paragraph = Paragraph::new(text)
            .block(block)
            .scroll((0, 0)) //TODO
            .fg(Color::Cyan)
            .bg(Color::Black);
        let paragraph_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height - 2,
        };

        paragraph.render(paragraph_area, buf);

        if content_height > 2 {
            let footer_y = area.y + area.height - 2; // last line inside border
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
