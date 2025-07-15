use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Write, stdout};
use std::path::Path;
use std::sync::Mutex;

use crate::{
    cli::CliOpt,
    event::{AppEvent, Event, EventHandler},
    filesbuffers::{FilesBuffers, Move},
    ui::FOOTER_SIZE,
};
use chrono::{DateTime, Local};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent},
};

pub const APP_NAME: &str = "Oxide";
pub const EMPTY_STRING: &str = "";

#[derive(Debug, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CursorType {
    Block,
    Line,
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Application.
#[derive(Debug)]
pub struct Editor {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Cli opts
    pub cli_opts: CliOpt,
    /// Files buffers
    pub buffers: FilesBuffers,
    /// Current file path
    pub current_file_path: String,
    /// Editor Mode
    pub editor_mode: EditorMode,
    /// Repetitions buffer
    pub repetitions: String,
    /// Command popup
    pub command_popup: CommandPopup,
    /// Saver lock
    lock: Mutex<()>,
    /// Last save date
    last_save: DateTime<Local>,
}

#[derive(Debug)]
pub struct CommandPopup {
    /// Is the popup shown ?
    pub running: bool,
    /// Command input field
    pub input_field: String,
}

impl Editor {
    /// Constructs a new instance of [`App`].
    pub fn new(cli_opts: CliOpt) -> Self {
        let mut current_file_path = EMPTY_STRING.to_string();
        let mut buffers = FilesBuffers::new();
        for file_path in cli_opts.file() {
            let mut actual_path = file_path.clone();
            if let Ok(file_path) = Path::new(file_path).canonicalize() {
                if let Some(file_path) = file_path.to_str() {
                    actual_path = file_path.to_string();
                }
            }
            if current_file_path == *EMPTY_STRING {
                current_file_path = actual_path;
            }
            buffers.init_file_buffer(current_file_path.to_string());
        }
        if buffers.is_empty() {
            buffers.init_file_buffer(EMPTY_STRING.to_string());
        }
        Self {
            running: true,
            events: EventHandler::new(),
            cli_opts,
            buffers,
            current_file_path,
            editor_mode: EditorMode::Normal,
            repetitions: EMPTY_STRING.to_string(),
            command_popup: CommandPopup {
                running: false,
                input_field: EMPTY_STRING.to_string(),
            },
            lock: Mutex::new(()),
            last_save: Local::now(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        Self::set_cursor_type(CursorType::Block);
        let terminal_height =
            terminal.size().expect("Terminal should have size").height - FOOTER_SIZE - 1;
        while self.running {
            terminal.draw(|frame| {
                frame.render_widget(&self, frame.area());
                let file_buffer = self.buffers.get_mut(self.current_file_path.clone());
                let cursor_position = file_buffer.to_cursor_position();
                frame.set_cursor_position((
                    cursor_position.x,
                    u16::min(cursor_position.y, terminal_height),
                ));
            })?;
            let file_buffer = self.buffers.get_mut(self.current_file_path.clone());
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        if self.command_popup.running {
                            self.handle_command_key_events(key_event)?
                        } else {
                            self.handle_key_events(key_event)?
                        }
                    }
                }
                Event::App(app_event) => {
                    let repetitions = self.repetitions.parse::<u16>().unwrap_or(1);
                    self.repetitions = EMPTY_STRING.to_string();
                    for _ in 0..repetitions {
                        match app_event {
                            AppEvent::Quit => self.running = false,
                            AppEvent::NormalMode => {
                                self.editor_mode = EditorMode::Insert;
                                Self::set_cursor_type(CursorType::Line);
                            }
                            AppEvent::InsertMode => {
                                self.editor_mode = EditorMode::Normal;
                                Self::set_cursor_type(CursorType::Block);
                            }
                            AppEvent::CreateLine => file_buffer.create_line(),
                            AppEvent::WriteAfterCursor(input) => file_buffer.insert_char(input),
                            AppEvent::DeleteBeforeCursor => file_buffer.delete_previous_position(),
                            AppEvent::MoveLeft => file_buffer.move_cursor(Move::Left),
                            AppEvent::MoveUp => file_buffer.move_cursor(Move::Up),
                            AppEvent::MoveRight => file_buffer.move_cursor(Move::Right),
                            AppEvent::MoveDown => file_buffer.move_cursor(Move::Down),
                            AppEvent::MoveToNextWord => file_buffer.move_to_next_word(),
                            AppEvent::MoveToPreviousWord => file_buffer.move_to_previous_word(),
                            AppEvent::CommandPopup => self.command_popup.running = true,
                            AppEvent::WriteInCommandInput(input) => {
                                self.command_popup.input_field += &input.to_string();
                            }
                            AppEvent::DeleteLastInCommandInput => {
                                if !self.command_popup.input_field.is_empty() {
                                    self.command_popup
                                        .input_field
                                        .remove(self.command_popup.input_field.len() - 1);
                                }
                            }
                            AppEvent::ExecuteCommand => {
                                //TODO : command processing
                            }
                        }
                    }
                    self.auto_save();
                }
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of command popup
    pub fn handle_command_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc => self.exit_popup(),
            KeyCode::Char(input) => {
                self.events.send(AppEvent::WriteInCommandInput(input));
            }
            KeyCode::Backspace => {
                self.events.send(AppEvent::DeleteLastInCommandInput);
            }
            KeyCode::Enter => {
                self.events.send(AppEvent::ExecuteCommand);
                self.exit_popup();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char(input)
                if self.editor_mode != EditorMode::Insert && input.is_ascii_digit() =>
            {
                // IMPORTANT: specific to this action :
                // Don't process it as an event so that repetitions can be processed seemlessly in
                // events processing
                self.repetitions += &input.to_string()
            }
            KeyCode::Char('i') if self.editor_mode == EditorMode::Normal => {
                self.events.send(AppEvent::NormalMode);
            }
            KeyCode::Char('a') if self.editor_mode == EditorMode::Normal => {
                self.events.send(AppEvent::MoveRight);
                self.events.send(AppEvent::NormalMode);
            }
            KeyCode::Char(input) if self.editor_mode == EditorMode::Insert => {
                self.events.send(AppEvent::WriteAfterCursor(input))
            }
            KeyCode::Esc if self.editor_mode == EditorMode::Insert => {
                self.events.send(AppEvent::InsertMode);
            }
            KeyCode::Esc if self.editor_mode == EditorMode::Normal => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Backspace if self.editor_mode == EditorMode::Insert => {
                self.events.send(AppEvent::DeleteBeforeCursor)
            }
            KeyCode::Enter if self.editor_mode == EditorMode::Insert => {
                self.events.send(AppEvent::CreateLine)
            }
            KeyCode::Left | KeyCode::Char('h') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::MoveLeft)
            }
            KeyCode::Up | KeyCode::Char('k') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::MoveUp)
            }
            KeyCode::Right | KeyCode::Char('l') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::MoveRight)
            }
            KeyCode::Down | KeyCode::Char('j') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::MoveDown)
            }
            KeyCode::Down | KeyCode::Char('w') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::MoveToNextWord)
            }
            KeyCode::Down | KeyCode::Char('b') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::MoveToPreviousWord)
            }
            KeyCode::Down | KeyCode::Char(':') if self.editor_mode != EditorMode::Insert => {
                self.events.send(AppEvent::CommandPopup)
            }
            /*KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }*/
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    fn set_cursor_type(cursor_type: CursorType) {
        let mut stdout = stdout();
        match cursor_type {
            CursorType::Block => {
                write!(stdout, "\x1b[2 q").unwrap();
                stdout.flush().unwrap();
            }
            CursorType::Line => {
                write!(stdout, "\x1b[6 q").unwrap();
                stdout.flush().unwrap();
            }
        }
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    fn exit_popup(&mut self) {
        self.command_popup.running = false;
        self.command_popup.input_field = EMPTY_STRING.to_string();
    }

    fn auto_save(&mut self) {
        let _guard = self.lock.lock().unwrap();
        let current = Local::now();
        let last_save_duration = current.naive_local() - self.last_save.naive_local();
        if last_save_duration.num_seconds() < 1 {
            return;
        };
        for (path, buf) in self.buffers.files.iter() {
            if path == EMPTY_STRING {
                continue;
            };
            let file_bytes = to_bytes(&buf.file);
            let bytes_slice: &[u8] = &file_bytes;
            let Ok(mut file) = File::create(path) else {
                continue;
            };
            if let Err(e) = file.write_all(bytes_slice) {
                log_error(&e.to_string());
            }
            if let Err(e) = file.flush() {
                log_error(&e.to_string());
            }
        }
        self.last_save = Local::now();
    }
}

fn to_bytes(buffer: &Vec<Vec<char>>) -> Vec<u8> {
    let mut result = String::new();

    for line in buffer {
        for c in line {
            result.push(*c);
        }
        result.push('\n'); // Optional: join lines with newlines
    }

    result.into_bytes() // returns Vec<u8>
}

fn log_error(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
        .unwrap();

    writeln!(file, "{}", msg).unwrap();
}
