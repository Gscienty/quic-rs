use crate::{
    attr::{Deserializer, Serializer},
    util,
};

use super::types::FrameType;

/// MAX_DATA 帧
///
/// 用于流量控制, 通知通信对方可以在整个连接上发送的最大数据量.
///
/// 帧结构如下:
/// MAX_DATA Frame {
///     Type (i) = 0x10,
///     Maximum Data (i),
/// }
pub struct MaxDataFrame {
    maximum_data: usize,
}

impl MaxDataFrame {
    /// 构造一个 MAX_DATA 帧
    ///
    /// # Returns
    /// 返回一个 MAX_DATA 帧
    pub fn new() -> Self {
        Self { maximum_data: 0 }
    }

    /// 获取发送的最大数据量
    ///
    /// # Returns
    /// 返回发送的最大数据量
    #[inline(always)]
    pub(crate) const fn get_maximum_data(&self) -> usize {
        self.maximum_data
    }

    /// 设置发送的最大数据量
    ///
    /// # Arguments
    /// `maximum_data` - 发送的最大数据量
    #[inline(always)]
    pub(crate) fn set_maximum_data(&mut self, maximum_data: usize) {
        self.maximum_data = maximum_data
    }
}

impl Serializer for MaxDataFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::MaxData.into()])?;

        payload_size += util::write_varint(self.maximum_data as u64, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for MaxDataFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let maximum_data = util::read_varint(r)?;
        self.maximum_data = maximum_data.value as usize;
        payload_size += maximum_data.size;

        Ok(payload_size)
    }
}
