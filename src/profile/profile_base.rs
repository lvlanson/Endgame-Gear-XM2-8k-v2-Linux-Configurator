use std::collections::HashMap;

use crate::profile::{
    profile_attribute::{
        ProfileAttribute, SingleBinaryAttributeHandler, SingleByteContinuousAttribute,
        SwitchAttributeHandler,
    },
    profile_attribute_args::{Range, Translation},
};
/// Profile object containing the profile buffer and
/// added functionality such as:
/// - init: creates empty profile (zero vec)
///
/// * `profile_buf`:
pub struct Profile {
    pub profile_buf: [u8; Self::PROFILE_SIZE],
    pub profile_fields: HashMap<ProfileFieldNames, ProfileAttribute>,
}
impl Profile {
    const PROFILE_SIZE: usize = 1041;

    pub fn init() -> Self {
        use ProfileFieldNames as PFN;
        // defining fields
        let poll_rate = ProfileAttribute {
            name: "Polling Rate".into(),
            description: 
                "Polling Rate is the frequency in which information is being exchanged between the computer and the mouse.\n
                Allowed values are [8, 4, 2, 1] and represent [100, 2000, 4000, 8000] Hz respectively.".into(),
            addresses: vec![21],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: Box::new(SingleBinaryAttributeHandler{
                translation: Translation{
                    code: vec![0x08, 0x04, 0x02, 0x01],
                    decode: vec![String::from("1000Hz"), String::from("2000Hz"), String::from("4000Hz"), String::from("8000Hz")]
                }
            }),
        };
        let slamclick_filter = ProfileAttribute {
            name: "Slamclick Filter".into(),
            description: 
                "Slamclick Filter filters out accidental clicks when the mouse is lifted and slammed down. When enabled non-intended mouseclicks will be filtered out.\nAllowed values are [0,1] which represent [OFF, ON] respectively".into(),
            addresses: vec![22],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: Box::new(SwitchAttributeHandler),
        };
        let disable_led_on_liftoff = ProfileAttribute {
            name: "Disable LED on Lift-Off".into(),
            description: 
                "Disables the bottom indicator LED when the mouse is lifted off.\nAllowed values are [0,1] which represent [OFF, ON] respectively".into(),
            addresses: vec![24],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: Box::new(SwitchAttributeHandler),
        };
        let liftoff_distance = ProfileAttribute {
            name: "LOD (Lift-Off Distance)".into(),
            description: 
                "Describes at which distance a Lift-Off is considered to be one.\nAllowed values are [0 - 10] which represent [0.7mm - 1.7mm] in 0.1mm steps.".into(), 
            addresses: vec![25],
            has_datafield: false,
            datafield_domain: None,
            datafield_addresses: None,
            attribute_handler: Box::new(SingleByteContinuousAttribute {
                range: Range {
                    decode_min: 0.7,
                    decode_step: 0.1,
                    code_min: 0x00,
                    code_step: 0x01,
                    code_max: 0x0a,
                    unit: "mm".into(),
                },
            }),
        };
        let angle_snapping = ProfileAttribute {
            name: "Angle Snapping".into(),
            description: 
                "Angle Snapping will ignore smaller jitters when moving horizontally or vertically and will straighten out the movement.\nAllowed values are [0,1] which represent [OFF, ON] respectively.".into(),
            addresses: vec![26],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: Box::new(SwitchAttributeHandler),
        };

        // adding fields to hashmap
        let profile_fields = HashMap::from_iter([
            (PFN::PollRate, poll_rate),
            (PFN::SlamclickFilter, slamclick_filter),
            (PFN::DisableLedOnLiftoff, disable_led_on_liftoff),
            (PFN::LiftoffDistance, liftoff_distance),
            (PFN::AngleSnapping, angle_snapping),
        ]);
        Self {
            profile_buf: [20; Self::PROFILE_SIZE],
            profile_fields: profile_fields,
        }
    }

    pub fn print_profile(&self) {
        for (_, attribute) in self.profile_fields.iter() {
            let data: Vec<u8> = attribute
                .addresses
                .iter()
                .map(|adr| self.profile_buf[(*adr) as usize])
                .collect();
            println!("{}: {}",attribute.name, attribute.attribute_handler.tostring(&data));
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
