use std::collections::HashMap;

use crate::profile::{
    profile_attribute::{
        AttributeHandler, ProfileAttribute, SingleBinaryAttributeHandler,
        SingleByteContinuousAttribute, SwitchAttribute,
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
    profile_fields: HashMap<ProfileFieldNames, ProfileAttribute>,
}
impl Profile {
    const PROFILE_SIZE: usize = 1041;

    pub fn init() -> Self {
        use ProfileFieldNames as PFN;
        // defining fields
        let poll_rate = ProfileAttribute {
            name: String::from("Polling Rate"),
            description: String::from(
                "Polling Rate is the frequency in which information is being exchanged between the computer and the mouse.\n
                Allowed values are [8, 4, 2, 1] and represent [100, 2000, 4000, 8000] Hz respectively.",
            ),
            addresses: vec![0x21],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: AttributeHandler::SingleBinaryAttributeHandler(SingleBinaryAttributeHandler::new(
                Translation{
                    code: vec![0x08, 0x04, 0x02, 0x01],
                    decode: vec![String::from("1000Hz"), String::from("2000Hz"), String::from("4000Hz"), String::from("8000Hz")]
                }
            )),
        };
        let slamclick_filter = ProfileAttribute {
            name: String::from("Slamclick Filter"),
            description: String::from(
                "Slamclick Filter filters out accidental clicks when the mouse is lifted and slammed down. When enabled non-intended mouseclicks will be filtered out.\nAllowed values are [0,1] which represent [OFF, ON] respectively",
            ),
            addresses: vec![0x22],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: AttributeHandler::SwitchAttribute(SwitchAttribute),
        };
        let disable_led_on_liftoff = ProfileAttribute {
            name: String::from("Disable LED on Lift-Off"),
            description: String::from(
                "Disables the bottom indicator LED when the mouse is lifted off.\nAllowed values are [0,1] which represent [OFF, ON] respectively",
            ),
            addresses: vec![0x24],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: AttributeHandler::SwitchAttribute(SwitchAttribute),
        };
        let liftoff_distance = ProfileAttribute {
            name: String::from("LOD (Lift-Off Distance)"),
            description: String::from(
                "Describes at which distance a Lift-Off is considered to be one.\nAllowed values are [0 - 10] which represent [0.7mm - 1.7mm] in 0.1mm steps.",
            ),
            addresses: vec![0x25],
            has_datafield: false,
            datafield_domain: None,
            datafield_addresses: None,
            attribute_handler: AttributeHandler::SingleByteContinuousAttribute(
                SingleByteContinuousAttribute::new(Range {
                    decode_min: 0.7,
                    decode_step: 0.1,
                    code_min: 0x00,
                    code_step: 0x01,
                    code_max: 0x0a,
                }),
            ),
        };
        let angle_snapping = ProfileAttribute {
            name: String::from("Angle Snapping"),
            description: String::from(
                "Angle Snapping will ignore smaller jitters when moving horizontally or vertically and will straighten out the movement.\nAllowed values are [0,1] which represent [OFF, ON] respectively.",
            ),
            addresses: vec![0x26],
            has_datafield: false,
            datafield_addresses: None,
            datafield_domain: None,
            attribute_handler: AttributeHandler::SwitchAttribute(SwitchAttribute),
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
            profile_buf: [0; Self::PROFILE_SIZE],
            profile_fields: profile_fields,
        }
    }

    fn print_profile(&self) {
        for (key, attribute) in self.profile_fields.iter() {
            let data: Vec<u8> = attribute
                .addresses
                .iter()
                .map(|adr| self.profile_buf[(*adr) as usize])
                .collect();
            // println!("{}", attribute.attribute_handler.to_string(data));
        }
    }

    fn update(&mut self, buf: &mut [u8; Self::PROFILE_SIZE]) {
        self.profile_buf.copy_from_slice(buf);
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
