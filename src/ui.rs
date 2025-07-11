use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::{APP_NAME, Editor};

impl Widget for &Editor {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(APP_NAME)
            .title_alignment(Alignment::Center);

        let (chars, buffer_position) = self.buffers.get(self.current_file_path.clone());

        let paragraph = Paragraph::new(String::from_iter(chars))
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black);

        paragraph.render(area, buf);
    }
}
