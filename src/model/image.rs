pub type RawImage = (u32, u32, Vec<u8>);

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}
