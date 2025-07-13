use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
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
        let block = Block::new();

        let file_buffer = self.buffers.get(self.current_file_path.clone()).file;
        let lines: Vec<String> = file_buffer
            .iter()
            .map(|line| line.iter().map(|c| c.to_string()).collect())
            .collect();
        let text = lines.join("\n");

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black);

        paragraph.render(area, buf);
    }
}
