use crate::{
    handler::Handler,
    profile::profile_base::Profile,
    view::{
        data::TableMouseOptions,
        layout_widgets::{FieldLabels, SelectionTable},
    },
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{
        Constraint::{Fill, Length},
        Layout,
    },
    text::Text,
    widgets::{Paragraph, Table, TableState, Wrap},
};

/// Helper for switching tables Mouseoptions <-> Mousevalues
enum Direction {
    Left,
    Right,
}

enum AppStatus {
    HandlerLoaded {
        handler: Handler,
        profile_err: String,
    },
    ProfileLoaded {
        handler: Handler,
        profile: Profile,
    },
    HandlerError(String),
    NotInitialized,
}

pub struct StateMemory {
    state: TableState,
    last_selected: Option<usize>,
}
/// Tui app parameters
///
/// * `exit`: tui running bool
/// * `handler`: handler for the mouse
/// * `options_table`: table displaying the mouse options
pub struct App {
    exit: bool,
    app_status: AppStatus,
    option_state: StateMemory,
    value_state: TableState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            app_status: AppStatus::NotInitialized,
            option_state: StateMemory {
                state: TableState::default().with_selected(0),
                last_selected: Some(0),
            },
            value_state: TableState::default().with_selected(None),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        // initializing mouse usb handler
        self.app_status = match Handler::init() {
            Err(err) => AppStatus::HandlerError(err),
            Ok(h) => match h.read_profile() {
                Ok(p) => AppStatus::ProfileLoaded {
                    handler: h,
                    profile: p,
                },
                Err(err) => AppStatus::HandlerLoaded {
                    handler: h,
                    profile_err: err,
                },
            },
        };

        // main loop
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        // general layout
        // horizontal
        let area = frame.area();
        let [vertical_layout, layout_values] =
            Layout::horizontal([Fill(1), Length(25)]).areas(area);

        // vertical
        let [layout_options, layout_infos] =
            Layout::vertical([Fill(1), Length(7)]).areas(vertical_layout);

        let block_options = FieldLabels::block_options();
        let block_values = FieldLabels::block_values();
        let block_infos = FieldLabels::block_infos();

        let p1: String = "test".into();
        let p2: String = "test2".into();
        let p3: String = "test3".into();

        let selection_widget = SelectionTable::new(vec![p1, p2, p3])
            .block(block_values)
            .row_highlight_style(TableMouseOptions::get_highlight_style());

        frame.render_stateful_widget(selection_widget, layout_values, &mut self.value_state);

        match &mut self.app_status {
            AppStatus::HandlerError(err) => {
                let para =
                    Paragraph::new(format!("Handler Failed with {}", err)).block(block_options);
                frame.render_widget(para, layout_options);
                frame.render_widget(block_infos, layout_infos);
            }
            AppStatus::HandlerLoaded {
                handler: _,
                profile_err,
            } => {
                let para =
                    Paragraph::new(format!("Mouse initialization failed with {}", profile_err))
                        .block(block_options);
                frame.render_widget(para, layout_options);
                frame.render_widget(block_infos, layout_infos);
            }
            AppStatus::ProfileLoaded {
                profile,
                handler: _,
            } => {
                // render options table
                let params = TableMouseOptions::params_from_profile(&profile);
                let table = Table::new(params.rows, params.widths)
                    .block(block_options)
                    .row_highlight_style(TableMouseOptions::get_highlight_style());
                frame.render_stateful_widget(table, layout_options, &mut self.option_state.state);

                // render options description
                let paragraph_desc = match self.option_state.state.selected_mut() {
                    Some(index) => {
                        Paragraph::new(Text::from(profile.option_description(index).clone()))
                            .wrap(Wrap { trim: false })
                            .block(block_infos)
                    }
                    None => Paragraph::new(Text::from(
                        profile
                            .option_description(&self.option_state.last_selected.unwrap())
                            .clone(),
                    ))
                    .wrap(Wrap { trim: false })
                    .block(block_infos),
                };
                frame.render_widget(paragraph_desc, layout_infos);
            }
            AppStatus::NotInitialized => {}
        }

        //list_options.render(layout_options, buf);
        // frame.render_widget(block_values, layout_values);
        // frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => match self.option_state.state.selected() {
                Some(_) => self.option_state.state.select_next(),
                None => self.value_state.select_next(),
            },
            KeyCode::Up => match self.option_state.state.selected() {
                Some(_) => self.option_state.state.select_previous(),
                None => self.value_state.select_previous(),
            },
            KeyCode::Right => self.switch_tables(Direction::Right),
            KeyCode::Left => self.switch_tables(Direction::Left),
            _ => {}
        }
    }

    fn switch_tables(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.value_state.select(None);
                self.option_state
                    .state
                    .select(self.option_state.last_selected);
            }
            Direction::Right => {
                match self.value_state.selected() {
                    Some(_) => {}
                    None => self.value_state.select(Some(0)),
                }
                match self.option_state.state.selected() {
                    Some(selected) => {
                        self.option_state.last_selected = Some(selected);
                        self.option_state.state.select(None);
                    }
                    None => {}
                }
            }
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
