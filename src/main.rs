pub mod handler;
pub mod profile;
use crate::handler::Handler;
use crate::profile::profile_base::Profile;

fn main() {
    let handler = Handler::init();
    let profile = handler.read_profile().unwrap();
    profile.print_profile();
}
