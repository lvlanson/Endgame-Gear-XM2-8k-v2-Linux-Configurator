use std::collections::HashMap;

use crate::profile::{profile_attribute::{DpiRangeHandler, KailhButtonFilterHandler, ProfileAttribute, SingleBinaryAttributeHandler, SingleByteContinuousAttribute, SwitchAttributeHandler}, profile_attribute_args::{Range, Translation}, profile_base::Profile};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ProfileFieldName {
    PollRate,
    SlamclickFilter,
    DisableLedOnLiftoff,
    LiftoffDistance,
    AngleSnapping,
    RippleControl,
    MotionSync,
    CpiLevels,
    CpiProf1,
    CpiProf2,
    CpiProf3,
    CpiProf4,
    LeftBtnMF,
    RightBtnMF,
    MidBtnMF,
    ForwardBtnMF,
    BackBtnMf,
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
    pub cpi_prof1: ProfileAttribute,
    pub cpi_prof2: ProfileAttribute,
    pub cpi_prof3: ProfileAttribute,
    pub cpi_prof4: ProfileAttribute,
    pub left_btn_mouse_filter: ProfileAttribute,
    pub right_btn_mouse_filter: ProfileAttribute,
    pub mid_btn_mouse_filter: ProfileAttribute,
    pub forward_btn_mouse_filter: ProfileAttribute,
    pub back_btn_mouse_filter: ProfileAttribute,
}
impl MouseProfile{

    pub fn new()-> Self{
        let range_click_filter = Range{
            code_min: 0x00,
            code_max: 0x19,
            code_step: 0x01,
            decode_min: 0.0,
            decode_step: 1.0,
            unit: "".into()
        };
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
            },
            cpi_prof1: ProfileAttribute{
                name: "DPI Setting for Profile 1".into(),
                description: "Sets the DPI for profile 1.\nAllowed values are [10-30.000] in steps of 10.".into(),
                addresses: vec![52,53,54,55],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(DpiRangeHandler),
            },          
            cpi_prof2: ProfileAttribute{
                name: "DPI Setting for Profile 2".into(),
                description: "Sets the DPI for profile 2.\nAllowed values are [10-30.000] in steps of 10.".into(),
                addresses: vec![57,58,59,60],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(DpiRangeHandler),
            },          
            cpi_prof3: ProfileAttribute{
                name: "DPI Setting for Profile 3".into(),
                description: "Sets the DPI for profile 3.\nAllowed values are [10-30.000] in steps of 10.".into(),
                addresses: vec![62,63,64,65],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(DpiRangeHandler),
            },          
            cpi_prof4: ProfileAttribute{
                name: "DPI Setting for Profile 4".into(),
                description: "Sets the DPI for profile 4.\nAllowed values are [10-30.000] in steps of 10.".into(),
                addresses: vec![67,68,69,70],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(DpiRangeHandler),
            },
            right_btn_mouse_filter: ProfileAttribute{
                name: "Multiclick Filter Right Mouse Button".into(),
                description: "Sets the level of filtering multiclicks. The front buttons support special modes GX Speed Mode and GX Safe Mode. GX Speed Mode registers at loss of contact for two contactpoints. GX Safe Mode registers at loss of one of those contactpoints as click. GX Safe Mode is good for worn down switches and is considered a hardware implemented double-click filter.\nAllowed values are [0-25], 240 or 241 which are the filter level, GX Safe Mode or GX Speed Mode respectively.".into(),
                addresses: vec![77],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(KailhButtonFilterHandler::new()),
            },
            left_btn_mouse_filter: ProfileAttribute{
                name: "Multiclick Filter Left Mouse Button".into(),
                description: "Sets the level of filtering multiclicks. The front buttons support special modes GX Speed Mode and GX Safe Mode. GX Speed Mode registers at loss of contact for two contactpoints. GX Safe Mode registers at loss of one of those contactpoints as click. GX Safe Mode is good for worn down switches and is considered a hardware implemented double-click filter.\nAllowed values are [0-25], 240 or 241 which are the filter level, GX Safe Mode or GX Speed Mode respectively.".into(),
                addresses: vec![84],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(KailhButtonFilterHandler::new())
            },
            mid_btn_mouse_filter: ProfileAttribute{
                name: "Multiclick Filter Middle Mouse Button".into(),
                description: "Sets the level of filtering multiclicks.\nAllowed values are [0-25].".into(),
                addresses: vec![91],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SingleByteContinuousAttribute::new(range_click_filter.clone())),
            },
            forward_btn_mouse_filter: ProfileAttribute{
                name: "Multiclick Filter Forward Mouse Button".into(),
                description: "Sets the level of filtering multiclicks.\nAllowed values are [0-25].".into(),
                addresses: vec![98],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SingleByteContinuousAttribute::new(range_click_filter.clone())),
            },
            back_btn_mouse_filter: ProfileAttribute{
                name: "Multiclick Filter Back Mouse Button".into(),
                description: "Sets the level of filtering multiclicks.\nAllowed values are [0-25].".into(),
                addresses: vec![105],
                has_datafield: false,
                datafield_addresses: None,
                attribute_handler: Box::new(SingleByteContinuousAttribute::new(range_click_filter.clone())),
            },

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
            (PFN::CpiProf1, &self.cpi_prof1),
            (PFN::CpiProf2, &self.cpi_prof2),
            (PFN::CpiProf3, &self.cpi_prof3),
            (PFN::CpiProf4, &self.cpi_prof4),
            (PFN::LeftBtnMF, &self.left_btn_mouse_filter),
            (PFN::RightBtnMF, &self.right_btn_mouse_filter),
            (PFN::MidBtnMF, &self.mid_btn_mouse_filter),
            (PFN::ForwardBtnMF, &self.forward_btn_mouse_filter),
            (PFN::BackBtnMf, &self.back_btn_mouse_filter),
        ])
    }
}
