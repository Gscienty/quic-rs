use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// PATH_RESPONSE 帧
///
/// 用于应答 PATH_CHALLENGE 帧.
///
/// 帧结构如下:
/// PATH_RESPONSE Frame {
///     Type (i) = 0x1a,
///     Data (64),
/// }
pub struct PathResponseFrame {
    data: [u8; 8],
}

impl PathResponseFrame {
    pub fn new() -> Self {
        Self { data: [0; 8] }
    }
}

impl Serializer for PathResponseFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::PathResponse.into()])?;

        w.write_all(&self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}

impl Deserializer for PathResponseFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        r.read_exact(&mut self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}
