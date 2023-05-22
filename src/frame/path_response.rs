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
pub(crate) struct PathResponseFrame {
    data: [u8; 8],
}

impl PathResponseFrame {
    /// 构造一个 PATH_RESPONSE 帧
    ///
    /// # Returns
    /// 返回一个 PATH_RESPONSE 帧
    pub(crate) fn new() -> Self {
        Self { data: [0; 8] }
    }

    /// 获取 PATH_RESPONSE 帧内的 Data
    ///
    /// # Returns
    /// 帧内 Data
    #[inline(always)]
    pub(crate) const fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// 设置 PATH_RESPONSE 帧内的 Data
    ///
    /// # Arguments
    /// `data` - 帧内 Data
    pub(crate) fn set_data(&mut self, data: &[u8]) {
        self.data.copy_from_slice(data)
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
