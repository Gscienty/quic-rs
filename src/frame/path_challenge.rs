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
    /// 构造一个 PATH_CHALLENGE 帧
    ///
    /// # Returns
    /// 返回一个 PATH_CHALLENGE 帧
    pub fn new() -> Self {
        Self { data: [0; 8] }
    }

    /// 获取 PATH_CHALLENGE 帧内的 Data
    ///
    /// # Returns
    /// 帧内 Data
    #[inline(always)]
    pub(crate) const fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// 设置 PATH_CHALLENGE 帧内的 Data
    ///
    /// # Arguments
    /// `data` - 帧内 Data
    pub(crate) fn set_data(&mut self, data: &[u8]) {
        self.data.copy_from_slice(data)
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
