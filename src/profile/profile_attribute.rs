use crate::profile::profile_attribute_args::{Range, Translation};

pub struct ProfileAttribute {
    pub name: String,
    pub description: String,
    pub addresses: Vec<u8>,
    pub has_datafield: bool,
    pub datafield_addresses: Option<Vec<u8>>,
    pub attribute_handler: Box<dyn ProfileAttributeHandler>,
}

pub trait ProfileAttributeHandler {
    fn validate(&self, data: &Vec<u8>) -> bool;
    fn tostring(&self, data: &Vec<u8>) -> String;
}

pub struct SwitchAttributeHandler;
impl ProfileAttributeHandler for SwitchAttributeHandler {
    fn validate(&self, data: &Vec<u8>) -> bool {
        data[0] == 0 || data[0] == 1
    }
    fn tostring(&self, data: &Vec<u8>) -> String {
        match data[0] {
            0 => String::from("OFF"),
            1 => String::from("ON"),
            _ => String::from("Not Supported"),
        }
    }
}

pub struct SingleByteContinuousAttribute {
    pub range: Range,
}
impl SingleByteContinuousAttribute {
    pub fn new(range: Range) -> Self {
        Self { range }
    }
}
impl ProfileAttributeHandler for SingleByteContinuousAttribute {
    fn validate(&self, data: &Vec<u8>) -> bool {
        let at_least_min = data[0] >= self.range.code_min;
        let at_most_max = data[0] <= self.range.code_max;
        let valid_val = (data[0] - self.range.code_min) % self.range.code_step == 0;
        at_least_min && at_most_max && valid_val
    }
    fn tostring(&self, data: &Vec<u8>) -> String {
        let delta = self.range.decode_min - (self.range.code_min as f32);
        let val = (data[0] as f32) * self.range.decode_step + delta;
        format!("{}{}", val.to_string(), self.range.unit)
    }
}

pub struct SingleBinaryAttributeHandler {
    pub translation: Translation,
}
impl SingleBinaryAttributeHandler {
    pub fn new(translation: Translation) -> Self {
        Self { translation }
    }
}
impl ProfileAttributeHandler for SingleBinaryAttributeHandler {
    fn validate(&self, data: &Vec<u8>) -> bool {
        // check if is power of 2 (n AND n-1)
        data[0] > 0 && (data[0] & (data[0] - 1) == 0)
    }
    fn tostring(&self, data: &Vec<u8>) -> String {
        match self.translation.code.iter().position(|&x| x == data[0]) {
            Some(index) => self.translation.decode[index].clone(),
            None => "value is not meaningful".into(),
        }
    }
}

pub struct DpiRangeHandler;
impl ProfileAttributeHandler for DpiRangeHandler {
    fn validate(&self, data: &Vec<u8>) -> bool {
        let min: u16 = 10;
        let max: u16 = 30_000;
        let step: u16 = 10;
        let left: u16 = (data[0] as u16) + (data[1] as u16) * ((16 as u32).pow(2) as u16);
        let right: u16 = (data[2] as u16) + (data[3] as u16) * ((16 as u32).pow(2) as u16);
        let left_valid: bool = left >= min && left <= max && left % step == 0;
        let right_valid: bool = right >= min && right <= max && right % step == 0;

        left_valid && right_valid
    }
    fn tostring(&self, data: &Vec<u8>) -> String {
        let left: u16 = (data[0] as u16) + (data[1] as u16) * ((16 as u32).pow(2) as u16);
        let right: u16 = (data[2] as u16) + (data[3] as u16) * ((16 as u32).pow(2) as u16);
        format!("{left}dpi {right}dpi")
    }
}

pub struct KailhButtonFilterHandler {
    range: Range,
    speed_mode: u8,
    safe_mode: u8,
}
impl KailhButtonFilterHandler {
    pub fn new() -> Self {
        Self {
            range: Range {
                code_min: 0x00,
                code_max: 0x19,
                code_step: 0x01,
                decode_min: 0.0,
                decode_step: 1.0,
                unit: "".into(),
            },
            safe_mode: 0xf0,
            speed_mode: 0xf1,
        }
    }
}
impl ProfileAttributeHandler for KailhButtonFilterHandler {
    fn validate(&self, data: &Vec<u8>) -> bool {
        let in_range: bool = data[0] >= self.range.code_min && data[1] <= self.range.code_max;
        in_range || data[0] == self.speed_mode || data[0] == self.safe_mode
    }
    fn tostring(&self, data: &Vec<u8>) -> String {
        let in_range = data[0] >= self.range.code_min && data[0] <= self.range.code_max;
        if in_range {
            format!("{}", data[0])
        } else if data[0] == self.speed_mode {
            "GX Speed Mode".into()
        } else if data[0] == self.safe_mode {
            "GX Safe Mode".into()
        } else {
            "Invalid Value Found".into()
        }
    }
}
