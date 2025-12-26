pub mod handler;
pub mod profile;
use crate::handler::Handler;
use crate::profile::profile_base::Profile;

fn main() {
    let handler = Handler::init();
    handler.read_profile();
}
