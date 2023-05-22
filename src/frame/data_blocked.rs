use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// DATA_BLOCKED 帧
///
/// 用于流量控制算法的调整输入.
/// 当发送方由于连接级流量控制而无法发送数据时，应发送 DATA_BLOCKED 帧.
///
/// 帧结构如下:
/// DATA_BLOCKED Frame {
///     Type (i) = 0x14,
///     Maximum Data (i),
/// }
pub struct DataBlockedFrame {
    /// 期望发送的最大数据量
    maximum_data: usize,
}

impl DataBlockedFrame {
    /// 构造一个 DATA_BLOCKED 帧
    ///
    /// # Returns
    /// 返回一个 DATA_BLOCKED 帧
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

impl Serializer for DataBlockedFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::DataBlocked.into()])?;

        payload_size += util::write_varint(self.maximum_data as u64, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for DataBlockedFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let maximum_data = util::read_varint(r)?;
        self.maximum_data = maximum_data.value as usize;
        payload_size += maximum_data.size;

        Ok(payload_size)
    }
}
