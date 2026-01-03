use std::collections::HashMap;

use crate::profile::{
    profile_attribute::{
        ProfileAttribute, SingleBinaryAttributeHandler, SingleByteContinuousAttribute,
        SwitchAttributeHandler,
    },
    profile_attribute_args::{Range, Translation},
    profile_fields::{MouseProfile, ProfileFieldName},
};
/// Profile object containing the profile buffer and
/// added functionality such as:
/// - init: creates empty profile (zero vec)
///
/// * `profile_buf`: holds the current profile readout, note [0] and [1] are read/write flags
/// * `profile_fields`: holds the mouseprofile with all the logic
pub struct Profile {
    pub profile_buf: [u8; Self::PROFILE_SIZE],
    pub profile_fields: MouseProfile,
    pub ordered_fields: [usize; 17],
}

impl Profile {
    const PROFILE_SIZE: usize = 1041;

    pub fn init() -> Self {
        use ProfileFieldName as PFN;
        Self {
            profile_buf: [20; Self::PROFILE_SIZE],
            profile_fields: MouseProfile::new(),
            ordered_fields: [
                PFN::POLLRATE,
                PFN::SLAMCLICKFILTER,
                PFN::DISABLELEDONLIFTOFF,
                PFN::LIFTOFFDISTANCE,
                PFN::ANGLESNAPPING,
                PFN::RIPPLECONTROL,
                PFN::MOTIONSYNC,
                PFN::CPILEVELS,
                PFN::CPIPROF1,
                PFN::CPIPROF2,
                PFN::CPIPROF3,
                PFN::CPIPROF4,
                PFN::LEFTBTNMF,
                PFN::RIGHTBTNMF,
                PFN::MIDBTNMF,
                PFN::FORWARDBTNMF,
                PFN::BACKBTNMF,
            ],
        }
    }

    pub fn print_profile(&self) {
        let field_map = self.profile_fields.hashmap();

        for field in &self.ordered_fields {
            let attribute: &ProfileAttribute = field_map.get(&field).unwrap();
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

    pub fn option_description(&self, index: &usize) -> &String {
        &self.profile_fields.hashmap()[index].description
    }
}
