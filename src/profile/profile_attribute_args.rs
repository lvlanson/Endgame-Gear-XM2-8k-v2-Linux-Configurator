pub struct Range {
    pub decode_min: f32,
    pub decode_step: f32,
    pub code_min: u8,
    pub code_step: u8,
    pub code_max: u8,
    pub unit: String,
}

pub struct Translation {
    pub code: Vec<u8>,
    pub decode: Vec<String>,
}
