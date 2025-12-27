use std::collections::HashMap;

use crate::profile::{
    profile_attribute::{
        ProfileAttribute, SingleBinaryAttributeHandler, SingleByteContinuousAttribute,
        SwitchAttributeHandler,
    },
    profile_attribute_args::{Range, Translation},
    profile_fields::MouseProfile,
};
/// Profile object containing the profile buffer and
/// added functionality such as:
/// - init: creates empty profile (zero vec)
///
/// * `profile_buf`:
pub struct Profile {
    pub profile_buf: [u8; Self::PROFILE_SIZE],
    pub profile_fields: MouseProfile,
}
impl Profile {
    const PROFILE_SIZE: usize = 1041;

    pub fn init() -> Self {
        Self {
            profile_buf: [20; Self::PROFILE_SIZE],
            profile_fields: MouseProfile::new(),
        }
    }

    pub fn print_profile(&self) {
        for (_, attribute) in self.profile_fields.hashmap().iter() {
            let data: Vec<u8> = attribute
                .addresses
                .iter()
                .map(|adr| self.profile_buf[(*adr) as usize])
                .collect();
            println!(
                "{}: {}",
                attribute.name,
                attribute.attribute_handler.tostring(&data)
            );
        }
    }

    fn update(&mut self, buf: &mut [u8; Self::PROFILE_SIZE]) {
        self.profile_buf.copy_from_slice(buf);
    }

    pub fn dump_hex(&self) {
        println!("{:02X?}", self.profile_buf)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum ProfileFieldNames {
    PollRate,
    SlamclickFilter,
    DisableLedOnLiftoff,
    LiftoffDistance,
    AngleSnapping,
}
