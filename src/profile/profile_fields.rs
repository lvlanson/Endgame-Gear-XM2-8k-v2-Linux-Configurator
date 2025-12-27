use std::collections::HashMap;

use crate::profile::{profile_attribute::{ProfileAttribute, SingleBinaryAttributeHandler, SingleByteContinuousAttribute, SwitchAttributeHandler}, profile_base::Profile, profile_attribute_args::{Range, Translation}};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ProfileFieldName {
    PollRate,
    SlamclickFilter,
    DisableLedOnLiftoff,
    LiftoffDistance,
    AngleSnapping,
    RippleControl,
    MotionSync,
    CpiLevels
}
pub struct MouseProfile{
    pub poll_rate: ProfileAttribute,
    pub slamclick_filter: ProfileAttribute,
    pub disable_led_on_liftoff: ProfileAttribute,
    pub liftoff_distance: ProfileAttribute,
    pub angle_snapping: ProfileAttribute,
    pub ripple_control: ProfileAttribute,
    pub motion_sync: ProfileAttribute,
    pub cpi_levels: ProfileAttribute,
}
impl MouseProfile{

    pub fn new()-> Self{
        Self {
            poll_rate:  ProfileAttribute {
                name: "Polling Rate".into(),
                description: 
                    "Polling Rate is the frequency in which information is being exchanged between the computer and the mouse.\n
                    Allowed values are [8, 4, 2, 1] and represent [100, 2000, 4000, 8000] Hz respectively.".into(),
                addresses: vec![21],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SingleBinaryAttributeHandler{
                    translation: Translation{
                        code: vec![0x08, 0x04, 0x02, 0x01],
                        decode: vec![String::from("1000Hz"), String::from("2000Hz"), String::from("4000Hz"), String::from("8000Hz")]
                    }
                }),
            },
            slamclick_filter: ProfileAttribute {
                name: "Slamclick Filter".into(),
                description: 
                    "Slamclick Filter filters out accidental clicks when the mouse is lifted and slammed down. When enabled non-intended mouseclicks will be filtered out.\nAllowed values are [0,1] which represent [OFF, ON] respectively".into(),
                addresses: vec![22],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SwitchAttributeHandler),
            },
            disable_led_on_liftoff: ProfileAttribute {
                name: "Disable LED on Lift-Off".into(),
                description: 
                    "Disables the bottom indicator LED when the mouse is lifted off.\nAllowed values are [0,1] which represent [OFF, ON] respectively".into(),
                addresses: vec![24],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SwitchAttributeHandler),
            },
            liftoff_distance:  ProfileAttribute {
                name: "LOD (Lift-Off Distance)".into(),
                description: 
                    "Describes at which distance a Lift-Off is considered to be one.\nAllowed values are [0 - 10] which represent [0.7mm - 1.7mm] in 0.1mm steps.".into(), 
                addresses: vec![25],
                has_datafield: false,
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
            },
           angle_snapping: ProfileAttribute {
                name: "Angle Snapping".into(),
                description: 
                    "Angle Snapping will ignore smaller jitters when moving horizontally or vertically and will straighten out the movement.\nAllowed values are [0,1] which represent [OFF, ON] respectively.".into(),
                addresses: vec![26],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SwitchAttributeHandler),
            },
            ripple_control: ProfileAttribute{
                name: "Ripple Control".into(),
                description: "Reducing jitter by applying smoothing for CPI>=1900. Side effects are a few less frames and added motion delay.\nAllowed values are [0,1] which represent [OFF, ON] respectively.".into(),
                addresses: vec![27],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SwitchAttributeHandler),
            },
            motion_sync: ProfileAttribute{
                name: "Motion Sync".into(),
                description: "Motion Sync synchronizes USB polling with frame reading. A new polling-frame will only be generated when it has been fetched. Input delay increases slightly (~1ms).\nAllowed values are [0,1] which represent [OFF, ON] respectively".into(),
                addresses: vec![28],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SwitchAttributeHandler),
            },
            cpi_levels: ProfileAttribute{
                name: "Number of CPI Levels".into(),
                description: "Sets how many CPI levels are available when cycling through CPI profiles.\nAllowed values are [1,2,3,4] which represent the number of CPI levels respectively".into(),
                addresses: vec![30],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SingleByteContinuousAttribute{
                    range: Range{
                        code_min: 1,
                        code_max: 4,
                        code_step: 1,
                        decode_min: 1.0,
                        decode_step: 1.0,
                        unit: " levels".into(),
                    }
                }),
            }
        }
    }
    pub fn hashmap(&self) -> HashMap<ProfileFieldName, &ProfileAttribute>{
        use ProfileFieldName as PFN;
        HashMap::from_iter([
            (PFN::PollRate, &self.poll_rate),
            (PFN::SlamclickFilter, &self.slamclick_filter),
            (PFN::DisableLedOnLiftoff, &self.disable_led_on_liftoff),
            (PFN::LiftoffDistance, &self.liftoff_distance),
            (PFN::AngleSnapping, &self.angle_snapping),
            (PFN::MotionSync, &self.motion_sync),
            (PFN::RippleControl, &self.ripple_control),
            (PFN::CpiLevels, &self.cpi_levels),
        ])
    }
}
