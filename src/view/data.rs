use std::collections::HashMap;

use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    text::Text,
    widgets::{Row, Table},
};

use crate::{
    handler::Handler,
    profile::{
        profile_attribute::ProfileAttribute,
        profile_base::Profile,
        profile_fields::{MouseProfile, ProfileFieldName},
    },
};

pub struct TableMouseOptions;
impl TableMouseOptions {
    pub fn params_from_profile(profile: &Profile) -> TableParams {
        let mut rows: Vec<Row> = vec![];
        let profile_map = profile.profile_fields.hashmap();
        for field in &profile.ordered_fields {
            let data = profile_map[&field]
                .addresses
                .iter()
                .map(|adr| profile.profile_buf[(*adr) as usize])
                .collect();
            rows.push(Row::new([
                Text::styled(profile_map[&field].name.clone(), Style::new().bold()),
                Text::styled(
                    profile_map[&field]
                        .attribute_handler
                        .tostring(&data)
                        .clone(),
                    Style::new().magenta(),
                ),
            ]))
        }
        let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];
        TableParams {
            rows: rows,
            widths: widths,
        }
    }
    pub fn initialize<'a>() -> TableParams<'a> {
        let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];
        TableParams {
            rows: vec![Row::new(vec!["Initializing...", ""])],
            widths: widths,
        }
    }
    pub fn get_highlight_style() -> Style {
        Style::new()
            .bg(Color::LightBlue)
            .fg(Color::Black)
            .bold()
            .italic()
    }
}

pub struct TableParams<'a> {
    pub rows: Vec<Row<'a>>,
    pub widths: [Constraint; 2],
}

#[derive(Debug, Default)]
pub enum MouseState {
    Disconnected,
    Connected,
    NotFound,
    #[default]
    NotInitialized,
}
