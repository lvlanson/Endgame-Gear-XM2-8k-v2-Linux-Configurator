use std::any::Any;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Row, StatefulWidget, Table, TableState},
};

pub struct FieldLabels;
impl FieldLabels {
    pub fn block_infos() -> Block<'static> {
        let title_info = Line::from(" Info ".bold()).left_aligned();

        Block::bordered()
            .title(title_info)
            .border_set(border::THICK)
            .yellow()
    }

    pub fn block_values() -> Block<'static> {
        let title_value = Line::from(" Values ".bold()).left_aligned();
        Block::bordered()
            .title(title_value)
            .border_set(border::THICK)
            .yellow()
    }

    pub fn block_options() -> Block<'static> {
        let sep = String::from("┃").bold();
        let title_option = Line::from(" Options ".bold()).right_aligned();
        let instructions = Line::from(vec![
            "┫".into(),
            " [Quit ".into(),
            "<Q>".blue().bold(),
            "] ".into(),
            sep.clone(),
            " [Up ".into(),
            "△".blue().bold(),
            "] ".into(),
            sep.clone(),
            " [Down ".into(),
            "▽".blue().bold(),
            "] ".into(),
            sep.clone(),
            " [Enter Option ".into(),
            "⎆".blue().bold(),
            "] ".into(),
            "┣".into(),
        ]);
        Block::bordered()
            .title(title_option)
            // .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .title_bottom(instructions)
            .yellow()
    }
}
// ▽▷▽△

enum AttributeType {}
enum ValueRange {
    Continuous {
        min: u8,
        max: u8,
        step: u8,
    },
    Fixed(Vec<u8>),
    Mixed {
        min: u8,
        max: u8,
        step: u8,
        fixed_vals: Vec<u8>,
    },
}
enum ValueDecode {
    Fixed(Vec<String>),
    Mixed {
        range: Vec<String>,
        fixed: Vec<String>,
    },
}
pub struct SelectionTable<'a> {
    cell_names: Vec<String>,
    cell_types: Vec<AttributeType>,
    cell_ranges: Vec<ValueRange>,
    cell_decode: Vec<ValueDecode>,
    block: Option<Block<'a>>,
    style: Option<Style>,
}

impl<'a> SelectionTable<'a> {
    pub fn new(cell_names: Vec<String>) -> Self {
        Self {
            cell_names,
            cell_types: vec![],
            cell_ranges: vec![],
            cell_decode: vec![],
            block: None,
            style: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn row_highlight_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<'a> StatefulWidget for SelectionTable<'a> {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TableState) {
        let mut row_widgets: Vec<Row> = vec![];
        let constraints = vec![Constraint::Fill(1)];
        for name in self.cell_names {
            row_widgets.push(Row::new(vec![name]));
        }
        let mut table: Table = match self.block {
            Some(block) => Table::new(row_widgets, constraints).block(block),
            None => Table::new(row_widgets, constraints),
        };

        table = match self.style {
            Some(style) => table.row_highlight_style(style),
            None => table,
        };

        StatefulWidget::render(table, area, buf, state);
    }
}
