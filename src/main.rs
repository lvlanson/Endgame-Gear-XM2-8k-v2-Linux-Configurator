pub mod handler;
pub mod profile;
pub mod view;
use crate::handler::Handler;
use crate::profile::profile_base::Profile;
use crate::view::app::App;
use crossterm::event::DisableMouseCapture;
use crossterm::execute;

fn main() -> std::io::Result<()> {
    let _ = execute!(std::io::stdout(), DisableMouseCapture);
    // let handler = Handler::init();
    // let profile = handler.read_profile().unwrap();
    // profile.print_profile();
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
