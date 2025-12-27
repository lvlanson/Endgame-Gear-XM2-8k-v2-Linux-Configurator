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
        use ProfileFieldName as PFN;
        let printout = [
            PFN::PollRate,
            PFN::SlamclickFilter,
            PFN::DisableLedOnLiftoff,
            PFN::LiftoffDistance,
            PFN::AngleSnapping,
            PFN::RippleControl,
            PFN::MotionSync,
            PFN::CpiLevels,
            PFN::CpiProf1,
            PFN::CpiProf2,
            PFN::CpiProf3,
            PFN::CpiProf4,
            PFN::LeftBtnMF,
            PFN::RightBtnMF,
            PFN::MidBtnMF,
            PFN::ForwardBtnMF,
            PFN::BackBtnMf,
        ];
        let field_map = self.profile_fields.hashmap();

        for field in printout {
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
}
