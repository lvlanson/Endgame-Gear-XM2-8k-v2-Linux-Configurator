use crate::profile::profile_attribute_args::{Range, Translation};

pub struct ProfileAttribute {
    pub name: String,
    pub description: String,
    pub addresses: Vec<u8>,
    pub has_datafield: bool,
    pub datafield_addresses: Option<Vec<u8>>,
    pub datafield_domain: Option<Vec<u8>>,
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
        let val = (data[0] as f32) * self.range.decode_step + self.range.decode_min;
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
    fn tostring(&self, _data: &Vec<u8>) -> String {
        // let index = self.translation.code.iter().find(data[0]);
        // self.translation.decode[index]
        String::from("to do")
    }
}
