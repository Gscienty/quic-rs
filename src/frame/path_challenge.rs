use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// PATH_CHALLENGE 帧
///
/// 用于检查与通信对方的可达性, 并在连接迁移期间进行路径验证.
///
/// 帧结构如下:
/// PATH_CHALLENGE Frame {
///     Type (i) = 0x1a,
///     Data (64),
/// }
pub struct PathChallengeFrame {
    data: [u8; 8],
}

impl PathChallengeFrame {
    pub fn new() -> Self {
        Self { data: [0; 8] }
    }
}

impl Serializer for PathChallengeFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::PathResponse.into()])?;

        w.write_all(&self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}

impl Deserializer for PathChallengeFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        r.read_exact(&mut self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}
